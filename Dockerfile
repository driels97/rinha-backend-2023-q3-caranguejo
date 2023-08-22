FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/local/bin

COPY --from=build /app/target/release/rinha-backend-2023-q3-caranguejo .

EXPOSE 80

CMD ["./rinha-backend-2023-q3-caranguejo"]
