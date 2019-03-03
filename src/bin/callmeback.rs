//extern crate callmeback;
extern crate dotenv;
extern crate env_logger;
extern crate listenfd;

use actix::prelude::*;
use actix_web::{server::HttpServer, App};
use callmeback::db::DbExecutor;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

pub fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut listenfd = ListenFd::from_env();
    let sys = actix::System::new("callmeback");

    let addr = SyncArbiter::start(3, move || DbExecutor::new(&database_url));

    let mut server = HttpServer::new(move || {
        callmeback::configure_app(App::with_state(callmeback::AppState { db: addr.clone() }))
    })
    .keep_alive(60)
    .shutdown_timeout(60);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };
    server.start();

    let _ = sys.run();
}
