extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_io;
extern crate tokio_proto;
extern crate futures;
extern crate bytes;

use std::io;

use bytes::BytesMut;

use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;
use futures::{future, Future, BoxFuture};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::TcpServer;
use tokio_service::Service;

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

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the `Item` type of the codec's `Encoder`
    type Request = String;

    /// For this protocol style, `Response` matches the `Item` type of the codec's `Decoder`
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
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

struct EchoRev;

impl Service for EchoRev {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars().rev().collect();
        future::ok(rev).boxed()
    }
}

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();

    let server = TcpServer::new(LineProto, addr);

    server.serve(|| Ok(EchoService));
}
