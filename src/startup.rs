use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in an Atomic Reference Counted pointer (ARC)
    // each instance of the application, instead of getting a raw copy
    // of a PgConnection, will get a pointer to one.
    // Arc<T> is alwasy clonable, no metter who T is. Cloning ARC increments
    // the number of active references and hads over a new copy of yhe memory address of the wrapped value.
    let db_pool = Data::new(db_pool);
    
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}