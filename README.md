# Who's this SuperGM?

A fun little quiz game about chess grandmasters implemented in Rust and htmx.

Tech stack:
- Backend: Rust with axum as the http server, minijinja as templating engine. The whole app is statically linked into one exectuable, including all templates, javascript, CSS and font files
- Frontend: plain HTML/JS/CSS + htmx to drive interaction via AJAX requests

## TODO

- Design results page
- Implement variable sizing based on document size, test on mobile devices
