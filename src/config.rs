use actix_web::{web, App, HttpServer, Responder};
use std::cell::Cell;

pub mod config {
    use std::cell::Cell;
    use actix_web::{Responder, web};

    #[derive(Clone)]
    pub struct Count {
        counter: Cell<usize>,
    }

    impl Count {
        pub fn new(counter: Cell<usize>) -> Self {
            Count { counter }
        }
    }

    async fn add_x(data: web::Data<Count>, path: web::Path<usize>) -> impl Responder {
        let count = data.counter.get();
        data.counter.set(count + path.into_inner());
        format!("counter: {}", data.counter.get())
    }

    async fn show(data: web::Data<Count>) -> impl Responder {
        format!("counter: {}", data.counter.get())
    }

    pub fn show_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/show")
                .route(web::to(show))
        );
    }

    pub fn add_config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/add/{x}")
                .route(web::to(add_x))
        );
    }
}