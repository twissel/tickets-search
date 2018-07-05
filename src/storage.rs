use actix::prelude::*;
use actix_web::Error;
use std::sync::{Arc, RwLock};
use ticket::{Ticket, Tickets};

use graph::TicketGraph;

use search::{search, SearchParams, Solutions};

// in memory storage with multithreaded access
pub struct NaiveStorage {
    graph: Arc<RwLock<TicketGraph>>,
}

impl NaiveStorage {
    pub fn new() -> Self {
        let graph = Arc::new(RwLock::new(TicketGraph::new()));
        NaiveStorage { graph }
    }

    pub fn insert_tickets(&self, tickets: Vec<Ticket>) {
        // shoud we lock the whole insert operation?
        let mut graph = self.graph.write().unwrap();
        for ticket in tickets {
            if !graph.contains_ticket(&ticket) {
                let id = ticket.id.clone();
                let from = graph.add_node_if_needed(&ticket.departure_code);
                let to = graph.add_node_if_needed(&ticket.arrival_code);
                graph.add_ticket_edge(from, to, ticket.into());
                graph.set_ticket_seen(id);
            }
        }
    }
}

impl Clone for NaiveStorage {
    fn clone(&self) -> Self {
        Self {
            graph: self.graph.clone(),
        }
    }
}

impl Actor for NaiveStorage {
    type Context = SyncContext<Self>;
}

impl Message for Tickets {
    type Result = Result<(), Error>;
}

impl Message for SearchParams {
    type Result = Result<Solutions, Error>;
}

impl Handler<Tickets> for NaiveStorage {
    type Result = Result<(), Error>;
    fn handle(&mut self, msg: Tickets, _: &mut Self::Context) -> Self::Result {
        self.insert_tickets(msg.tickets);
        Ok(())
    }
}

impl Handler<SearchParams> for NaiveStorage {
    type Result = Result<Solutions, Error>;
    fn handle(&mut self, msg: SearchParams, _: &mut Self::Context) -> Self::Result {
        let graph = self.graph.read().unwrap();
        search(&graph, &msg)
    }
}
