#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate futures_cpupool;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use futures_cpupool::CpuPool;
use futures::future::BoxFuture;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};

use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_minihttp::{Request, Response};
use rand::Rng;

struct Server {
    thread_pool: CpuPool,
    db_pool: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        unimplemented!();
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let thread_pool = CpuPool::new(10);

    let db_url = "postgres://postgres@localhost";
    let db_config = r2d2::Config::default();
    let db_manager = PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    let db_pool = r2d2::Pool::new(db_config, db_manager).unwrap();

    TcpServer::new(tokio_minihttp::Http, addr).serve(move || {
        Ok(Server {
            thread_pool: thread_pool.clone(),
            db_pool: db_pool.clone(),
        })
    });
}
