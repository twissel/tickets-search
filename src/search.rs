use actix_web::Error;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::Duration;
use chrono::{DateTime, Utc};
use graph::TicketGraph;
use petgraph::visit::EdgeRef;
use ticket::{Code, Id};

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    pub departure_code: Code,
    pub arrival_code: Code,

    #[serde(deserialize_with = "from_ts")]
    pub departure_time_start: DateTime<Utc>,

    #[serde(deserialize_with = "from_ts")]
    pub departure_time_end: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Solution {
    ticket_ids: (Id, Id),
    price: u64,
}

#[derive(Serialize)]
pub struct Solutions {
    solutions: Vec<Solution>,
}

pub fn search(graph: &TicketGraph, params: &SearchParams) -> Result<Solutions, Error> {
    let departure_time_start = params.departure_time_start;
    let departure_time_end = params.departure_time_end;
    let start = graph.get_node_indx_by_code(&params.departure_code);
    let goal = graph.get_node_indx_by_code(&params.arrival_code);
    match (start, goal) {
        // we have arrival and destination in our graph, let's search for tickets
        (Some(start), Some(goal)) => {
            let solutions = graph
                // find all tickets from departure airport to any other(excluding final destination) airport,  that match time criteria
                .outgoing_edges(start)
                .filter(|ticket| ticket.target() != goal)
                .filter(|ticket| {
                    let weight = ticket.weight();
                    (weight.departure_time >= departure_time_start
                        && weight.departure_time <= departure_time_end)
                })
                // find all matching tickets from docking airport to final destination
                .filter_map(|first_ticket| {
                    let first_dst = first_ticket.target();
                    let first_weight = first_ticket.weight();
                    let first_arrival_time = first_weight.arrival_time;
                    let min_wait_time = first_arrival_time + Duration::hours(3);
                    let max_wait_time = first_arrival_time + Duration::hours(8);
                    

                    let current_solutions = graph
                        .outgoing_edges(first_dst)
                        .filter(|edge| edge.target() == goal)
                        .filter(|edge| {
                            let second_departure_time = edge.weight().departure_time;
                            (second_departure_time > min_wait_time
                                && second_departure_time < max_wait_time)
                        })
                        .map(|edge| Solution {
                            ticket_ids: (first_weight.id.clone(), edge.weight().id.clone()),
                            price: first_weight.price + edge.weight().price,
                        })
                        .collect::<Vec<_>>();
                    if current_solutions.len() > 0 {
                        Some(current_solutions)
                    } else {
                        None
                    }
                })
                .fold(Vec::new(), |mut all, solutions| {
                    all.extend(solutions);
                    all
                });

            Ok(Solutions { solutions })
        }
        // probably we should return error here
        _ => Ok(Solutions {
            solutions: Vec::new(),
        }),
    }
}
