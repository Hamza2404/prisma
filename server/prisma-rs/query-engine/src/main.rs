use actix::System;
use actix_web::{http::Method, server, App, HttpRequest, Json, Responder};
use lazy_static::lazy_static;
use std::env;

mod dependencies;
use dependencies::ServerDependencies;

mod req_handlers;
use req_handlers::{GraphQlBody, PrismaRequest, RequestHandler};

lazy_static! {
    pub static ref DEPS: ServerDependencies = ServerDependencies::new();
}

fn handler((json, req): (Json<Option<GraphQlBody>>, HttpRequest)) -> impl Responder {
    dbg!("Calling `handler`");
    let req: PrismaRequest<GraphQlBody> = (json.clone().unwrap(), req).into();
    DEPS.request_handler.handle(req);

    // todo return values
    ""
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("prisma");

    server::new(|| App::new().resource("/", |r| r.method(Method::POST).with(handler)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8000");
    let _ = sys.run();
}