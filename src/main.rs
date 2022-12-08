use std::cell::Cell;
use actix_web::{App, HttpServer, web};
use crate::config::config::add_config;
use crate::config::config::show_config;
use crate::config::config::Count;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Count::new(Cell::new(0));

    HttpServer::new(move || {
        App::new()
            .configure(add_config)
            .configure(show_config)
            .app_data(web::Data::new(counter.clone()))
    }).bind(("127.0.0.1", 8080))?.run().await
}
