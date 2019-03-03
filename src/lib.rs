#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate actix_web_requestid;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod controller;
pub mod db;
mod models;
mod schema;

use self::db::DbExecutor;
use actix::prelude::*;
use actix_web::middleware::Logger;
use actix_web::{http, App};
use actix_web_requestid::RequestIDHeader;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub fn configure_app(app: App<AppState>) -> App<AppState> {
    app.middleware(RequestIDHeader)
        .middleware(Logger::default())
        .resource("/{datetime}", |r| {
            r.method(http::Method::POST).with_async_config(
                controller::callback::create,
                |(json_cfg,)| {
                    json_cfg.1.limit(4096);
                },
            );
        })
        .resource("/callback/{id}", |r| {
            r.method(http::Method::GET).with(controller::callback::get)
        })
}
