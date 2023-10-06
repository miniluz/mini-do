FROM rust:1.72 AS setup
# Install cargo-chef
RUN cargo install cargo-chef
WORKDIR /app

FROM setup AS planner
COPY . .
# Prepare needed libraries and save to recipe.json
RUN cargo chef prepare --recipe-path recipe.json

FROM setup as builder
# Build libraries
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build code
COPY . .
RUN cargo build --release

FROM setup as runtime
# Copy only compiled app
COPY --from=builder /app/target/release/migration /
ENTRYPOINT [ "/migration" ]