extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::stream::Stream;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

fn main() {
    let mut core = Core::new().unwrap();
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle()).unwrap();

    let connections = listener.incoming();
    // this processes all the connections as a Stream, but in order
    // and without concurrency
    // let welcomes = connections.and_then(|(socket, _peer_addr)| {
    //     return tokio_io::io::write_all(socket, b"Hello, world!\n");
    // });

    // let server = welcomes.for_each(|(_socket, _welcome)| {
    //     Ok(())
    // });

    // handle them concurrently, process all chained ops before taking the next socket.
    let handle = core.handle();
    let server = connections.for_each(|(socket, _peer_addr)| {
        let serve_one = tokio_io::io::write_all(socket, b"Hello, world\n").then(|_| Ok(()));
        handle.spawn(serve_one); // move work into event loop, concurrent, single threaded.
        // to move the futures onto M threads, we would need CpuPool
        Ok(())
    });

    core.run(server).unwrap();
}
