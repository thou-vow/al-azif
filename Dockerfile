FROM rustlang/rust:nightly AS builder

WORKDIR /bot

COPY . .

RUN cargo build --release

# The runtime container
FROM debian:bookworm-slim

COPY --from=builder /bot/target/release/al_azif /usr/local/bin/al_azif

CMD ["/usr/local/bin/al_azif"]