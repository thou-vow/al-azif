FROM lukemathwalker/cargo-chef:latest-rust-bullseye AS chef

WORKDIR /al_azif


# Stage 1: Recipe
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# Stage 2: Build
FROM chef AS builder

COPY --from=planner /al_azif/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin al_azif


# Stage 3: Runtime
FROM debian:bookworm-slim


COPY --from=builder /al_azif/target/release/al_azif /usr/local/bin/al_azif

CMD ["/usr/local/bin/al_azif"]