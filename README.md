# Who's this SuperGM?

A fun little quiz game about chess grandmasters implemented in Rust and htmx.

Tech stack:
- Backend: Rust with axum as the http server, minijinja as templating engine. The whole app is statically linked into one exectuable, including all templates, javascript, CSS and font files
- Frontend: plain HTML/JS/CSS + htmx to drive interaction via AJAX requests

## Development

To watch the `backend` executable for changes use
```
cargo watch -- cargo run --bin backend
```

## Benchmarking

For benchmarking, rewrk is a useful tool: https://github.com/lnx-search/rewrk. To benchmark the `/prediction` endpoint use
```
rewrk -c 256 -t 12 -d 15s -h "http://localhost:8080/prediction?game_id=1337&name=carlsen" --pct
```

## Deployment

If you are on Windows, first prepare for cross-compilation to Linux x86: https://github.com/KodrAus/rust-cross-compile

1. Build a release target for x86_64-unknown-linux-musl: `cargo build --target x86_64-unknown-linux-musl --release`
2. Run `fly launch`

## TODO

- Implement variable sizing based on document size, test on mobile devices
- Design results page
- Make order of categories more variable
- Try to reduce font download size
- Host images in /assets, scale them down to a reasonable size (300px height?)
