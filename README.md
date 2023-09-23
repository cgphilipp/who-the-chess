# Who's this SuperGM?

A fun little quiz game about chess grandmasters implemented in Rust and htmx.

Tech stack:
- Backend: Rust with axum as the http server, minijinja as templating engine. The whole app is statically linked into one exectuable, including all templates, javascript, CSS and font files
- Frontend: plain HTML/JS/CSS + htmx to drive interaction via AJAX requests

## Deployment

1. Build a release target for x86_64-unknown-linux-musl: `cargo build --target x86_64-unknown-linux-musl --release`
2. Run `fly launch`

## TODO

- Design results page
- Implement variable sizing based on document size, test on mobile devices
