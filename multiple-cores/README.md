# Run Reactor on mutliple threads

This server is another echo server, but spins up a new event loop for the number of cpu's available and provisions a `TcpListener` for each.

# Using
```bash
cargo run 127.0.0.1:6000
```

in another terminal:
```bash
telnet 127.0.0.1:6000
hello<enter>
```


