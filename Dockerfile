FROM rust:1.71.0 as builder

WORKDIR /app
COPY . .
RUN ./scripts/build.sh


FROM scratch as exporter
COPY --from=builder /app/out/ .

# docker build --output out .
