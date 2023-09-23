FROM debian:bullseye-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# copy cross-compiled backend directly to the server
COPY ./target/x86_64-unknown-linux-musl/release/backend /app/whos-this-gm

# Run the app
CMD ./whos-this-gm
