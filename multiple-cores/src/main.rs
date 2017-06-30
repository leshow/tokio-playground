#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate num_cpus;
extern crate tokio_core;
extern crate tokio_io;

use std::env;
use std::io::BufReader;
use std::net::{SocketAddr, TcpListener};
use std::thread;

use futures::{Future, Stream};
use tokio_core::net::TcpListener as TokTcpListener;
use tokio_core::reactor::Core;
use tokio_io::{io, AsyncRead};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Parse(std::net::AddrParseError);
    }

    errors {
        MissingArgument {
            description("Missing argument (the address to listen on)")
            display("Missing argument (the address to listen on)")
        }
        ThreadPanic(panic: String) {
            description("A thread panicked")
            display("A thread panicked: {}", panic)
        }
    }
}

fn run_thread(listener: TcpListener) -> Result<()> {
    let mut core = Core::new()?;
    let addr = listener.local_addr()?;
    let handle = core.handle();
    let listener = TokTcpListener::from_listener(listener, &addr, &handle)?;
    let incoming = listener.incoming();

    let all_conns = incoming.for_each(|(conn, addr)| {
        let (input, output) = conn.split();
        let input = BufReader::new(input);
        let lines = io::lines(input);

        let all_written = lines
            .fold(output, |output, mut line| {
                line += "\n";
                io::write_all(output, line).map(|(output, _line)| output)
            })
            .and_then(|output| io::shutdown(output))
            .map(|_closed| ())
            .map_err(move |e| println!("Lost connection {}: {}", addr, e));

        handle.spawn(all_written);
        Ok(())
    });

    core.run(all_conns)?;
    Ok(())
}

fn run() -> Result<()> {
    let str_addr = env::args().nth(1).ok_or(ErrorKind::MissingArgument)?;
    let addr = str_addr.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr)?;

    let mut threads = Vec::new();
    for _ in 0..num_cpus::get() {
        let listener = listener.try_clone()?;
        threads.push(thread::spawn(move || run_thread(listener).unwrap()));
    }

    for t in threads {
        t.join()
            .map_err(|e| ErrorKind::ThreadPanic(format!("{:?}", e)))?;
    }

    Ok(())
}

quick_main!(run);
