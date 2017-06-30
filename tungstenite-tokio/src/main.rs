extern crate futures;
extern crate tokio_core;
extern crate tokio_tungstenite;
extern crate tungstenite;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};

use futures::stream::Stream;
use futures::Future;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tungstenite::protocol::Message;

use tokio_tungstenite::accept_async;

fn main() {
    let addr = "127.0.0.1:12345".parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let socket = TcpListener::bind(&addr, &handle).unwrap();

    let conns = Arc::new(Mutex::new(HashMap::new()));

    let srv = socket.incoming().for_each(|(stream, addr)| {
        let conns_inner = conns.clone();
        let handle_inner = handle.clone();

        accept_async(stream)
            .and_then(move |ws_stream| {
                println!("New websocket connection: {}", addr);
                let (tx, rx) = futures::sync::mpsc::unbounded();
                {
                    let mut c = conns_inner.lock().unwrap();
                    c.insert(addr, tx);
                }
                let (sink, stream) = ws_stream.split();

                let conns = conns_inner.clone();

                let ws_reader = stream.for_each(move |message: Message| {
                    println!("Received a message from {}: {}", addr, message);

                    let mut conns = conns.lock().unwrap();
                    let iter = conns
                        .iter_mut()
                        .filter(|&(k, _)| *k != addr)
                        .map(|(_, v)| v);
                    for tx in iter {
                        tx.send(message.clone()).unwrap();
                    }
                    Ok(())
                });
                let ws_writer = rx.fold(sink, |mut sink, msg| {
                    use futures::Sink;
                    sink.start_send(msg).unwrap();
                    Ok(sink)
                });

                let conn = ws_reader
                    .map(|_| ())
                    .map_err(|_| ())
                    .select(ws_writer.map(|_| ()).map_err(|_| ()));
                    
                handle_inner.spawn(conn.then(move |_| {
                    let mut c = conns_inner.lock().unwrap();
                    c.remove(&addr);
                    Ok(())
                }));
                Ok(())
            })
            .map_err(|e| Error::new(ErrorKind::Other, e))

    });

    core.run(srv).unwrap();
}
