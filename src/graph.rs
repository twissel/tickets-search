use ticket::{Code, Id, Ticket};

use chrono::{DateTime, Utc};
use petgraph::prelude::NodeIndex;
use petgraph::stable_graph::{Edges, StableDiGraph as DiGraph};
use petgraph::{Directed, Direction};
use std::collections::{HashMap, HashSet};

// just a wrapper around Digraph
pub struct TicketGraph {
    // NOTE: petgraph::graphmap::GraphMap does not allow parallel edges, thats why we are using Digraph here
    graph: DiGraph<(), TicketEdge>,
    code_to_indx: HashMap<Code, NodeIndex>,
    tickets_seen: HashSet<Id>,
}

impl TicketGraph {
    pub fn new() -> Self {
        let graph = DiGraph::new();
        let code_to_indx = HashMap::new();
        let tickets_seen = HashSet::new();
        Self {
            graph,
            code_to_indx,
            tickets_seen,
        }
    }
    pub fn add_node_if_needed(&mut self, node_code: &Code) -> NodeIndex {
        let graph = &mut self.graph;
        let entry = self.code_to_indx
            .entry(node_code.clone())
            .or_insert_with(|| graph.add_node(()));
        *entry
    }

    pub fn add_ticket_edge(&mut self, from: NodeIndex, to: NodeIndex, edge: TicketEdge) {
        self.graph.add_edge(from, to, edge);
    }

    pub fn contains_ticket(&self, ticket: &Ticket) -> bool {
        self.tickets_seen.contains(&ticket.id)
    }

    pub fn set_ticket_seen(&mut self, id: Id) {
        self.tickets_seen.insert(id);
    }

    pub fn get_node_indx_by_code(&self, code: &Code) -> Option<NodeIndex> {
        self.code_to_indx.get(code).map(|c| *c)
    }

    pub fn outgoing_edges(&self, node: NodeIndex) -> Edges<TicketEdge, Directed> {
        self.graph.edges_directed(node, Direction::Outgoing)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct TicketEdge {
    pub id: Id,
    pub departure_time: DateTime<Utc>,
    pub arrival_time: DateTime<Utc>,
    pub price: u64,
}

impl From<Ticket> for TicketEdge {
    fn from(ticket: Ticket) -> TicketEdge {
        TicketEdge {
            id: ticket.id,
            departure_time: ticket.departure_time,
            arrival_time: ticket.arrival_time,
            price: ticket.price,
        }
    }
}
