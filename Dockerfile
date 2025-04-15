# Step 1: Use the official Rust image for building
FROM rust:latest AS builder

# Step 2: Set the working directory inside the container
WORKDIR /usr/src/app

# Step 3: Copy the Cargo.toml and Cargo.lock to download dependencies separately for caching
COPY Cargo.toml Cargo.lock .env ./

# Step 4: Create an empty src directory to satisfy Cargo's build requirements
RUN mkdir src

# Step 5: Update dependencies to their latest compatible versions
# Step 5: Download the dependencies (layer caching will skip this if no changes in dependencies)
RUN cargo update
RUN cargo fetch

# Step 6: Copy the entire project source code
COPY . .

# Step 7: Build the application in release mode
RUN cargo build --release
#RUN cargo build --release --target=x86_64-unknown-linux-musl

# Step 8: Create a smaller final image based on a lightweight Linux image
FROM ubuntu:22.04

# Step 9: Copy files
COPY --from=builder /usr/src/app/target/release/dnp3_outstation /usr/local/bin/dnp3_outstation
COPY --from=builder /usr/src/app/.env /usr/local/bin/.env

# Step 10: Expose any ports your application uses (adjust the port as needed)
#EXPOSE 8080

# Step 11: Specify the command to run the application
CMD ["dnp3_outstation"]