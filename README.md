# Guessing game server

The guessing game from the Rust Book, but playable over a Unix socket.

Run the server with:

```bash
cargo run
```

Then in another terminal, start a game with:

```bash
socat - UNIX-CLIENT:/tmp/guessing_game_server.sock
```

You can start multiple games in parallel.