extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate futures;
extern crate petgraph;

extern crate serde;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix::prelude::*;
use actix_web::{http::Method, server, App, Error, Json, State};
use futures::future::{ok as fut_ok, result as fut_result};
use futures::prelude::*;

mod graph;
mod search;
mod storage;
mod ticket;

use search::{SearchParams, Solutions};
use storage::NaiveStorage;

use ticket::Tickets;

pub struct AppState {
    pub storage: Addr<Syn, NaiveStorage>,
}

fn import(
    (tickets, state): (Json<Tickets>, State<AppState>),
) -> impl Future<Item = Json<serde_json::Value>, Error = Error> {
    // do not wait for insertion to finish, just return "Ok" for now
    state.storage.do_send(tickets.into_inner());
    fut_ok(Json(json!({
                "status": "success"
            })))
}

fn search(
    (params, state): (Json<SearchParams>, State<AppState>),
) -> impl Future<Item = Json<Solutions>, Error = Error> {
    state
        .storage
        .send(params.into_inner())
        .from_err()
        .and_then(fut_result)
        .map(Json)
}

pub fn setup_app(app: App<AppState>) -> App<AppState> {
    app.resource("/batch_insert", |r| {
        r.method(Method::POST)
            .with_async(import)
            .0
            .limit(52_428_800); // allow large json inputs (50mb)
    }).resource("/search", |r| r.method(Method::POST).with_async(search))
}

fn main() {
    let sys = actix::System::new("tickets-search");
    let storage = NaiveStorage::new();

    let actor = SyncArbiter::start(4, move || storage.clone());
    server::new(move || {
        let app = App::with_state(AppState {
            storage: actor.clone(),
        });
        setup_app(app)
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: http://127.0.0.1:8080");
    let _ = sys.run();
}
