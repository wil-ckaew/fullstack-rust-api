# Build stage
FROM rust:1.79-buster as builder

WORKDIR /app

# Set environment variable for build time, if needed
ARG DATABASE_URL

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

# Copy the built application from the builder stage
COPY --from=builder /app/target/release/backend .

# Run the application
CMD ["./backend"]
