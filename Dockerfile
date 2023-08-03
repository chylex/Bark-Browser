FROM rust:1.71.0 as builder

WORKDIR /app
COPY . .
RUN cargo build --release


FROM scratch as exporter
COPY --from=builder /app/target/release/bark .

# docker build --output out .
