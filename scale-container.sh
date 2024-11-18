#!/bin/bash

# Base variables
BASE_NAME="dnp3_outstation_rust"
DOCKERFILE_PATH="."
NUM_CONTAINERS=10
NETWORK="scada-network"
BASE_IP_ADDR="10.0.11"
BASE_PORT=11180

# Step 1: Build the Docker image
echo "Building Docker image: $BASE_NAME"
sudo docker build -t "$BASE_NAME" "$DOCKERFILE_PATH"

if [ $? -ne 0 ]; then
  echo "Docker build failed! Exiting."
  exit 1
fi

# Step 2: Create containers with incrementing names
for i in $(seq 1 $NUM_CONTAINERS); do
  CONTAINER_NAME="${BASE_NAME}_${i}"
  # Incrementing IP address
  IP_ADDR="${BASE_IP_ADDR}.$((180 + i))"

  # Incrementing port
  PORT=$((BASE_PORT + i))

  echo "Creating container: $CONTAINER_NAME with IP $IP_ADDR and port $PORT"

  sudo docker run -d \
    --name "$CONTAINER_NAME" \
    --hostname "$CONTAINER_NAME" \
    --network "$NETWORK" \
    --ip "$IP_ADDR" \
    -p "$PORT:777" \
    --env-file .env \
    "$BASE_NAME"

  if [ $? -ne 0 ]; then
    echo "Failed to create container: $CONTAINER_NAME"
  fi
done

echo "All containers created!"
