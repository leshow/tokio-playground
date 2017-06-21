extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_io;
extern crate tokio_proto;
extern crate futures;
extern crate bytes;

use std::io;

use bytes::BytesMut;

use tokio_io::codec::{Encoder, Decoder};
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;
use tokio_core::net::TcpListener;
use tokio_service::{Service, NewService};
use futures::{future, Future, Stream, Sink, BoxFuture};

pub struct LineCodec;

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(item.as_bytes());
        dst.extend(b"\n");
        Ok(())
    }
}

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = src.iter().position(|b| *b == b'\n') {
            let b_msg = src.split_to(i);
            src.split_to(1);

            match std::str::from_utf8(b_msg.as_ref()) {
                Ok(s) => Ok(Some(s.to_owned())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "utf-8 error")),
            }
        } else {
            Ok(None)
        }
    }
}

fn serve<S>(s: S) -> io::Result<()>
where
    S: NewService<Request = String, Response = String, Error = io::Error> + 'static,
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle)?;

    let conns = listener.incoming();
    let server = conns.for_each(move |(stream, _addr)| {
        let (writer, reader) = stream.framed(LineCodec).split();
        let service = s.new_service()?;

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses).then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(server)
}

struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<String, io::Error>;
    fn call(&self, input: String) -> Self::Future {
        future::ok(input).boxed()
    }
}

fn main() {
    if let Err(e) = serve(|| Ok(EchoService)) {
        println!("Server exited with error {}", e);
    }
}
