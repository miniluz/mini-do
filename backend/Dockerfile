FROM rust:1.72 AS chef
# Install cargo-chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# Prepare needed libraries and save to recipe.json
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
# Build libraries
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build code
COPY . .
RUN cargo build --release --bin backend

FROM chef as runtime
# Copy only compiled app
COPY --from=builder /app/target/release/backend /
ENTRYPOINT [ "/backend" ]