# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/capture-api .

EXPOSE 3000

CMD ["./capture-api"]