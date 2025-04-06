#!/bin/bash

# Start Docker containers
echo "Starting Kafka containers..."
docker-compose up -d

# Wait for services to be healthy
echo "Waiting for services to be healthy..."
max_attempts=30
attempt=0
healthy=false

while [ "$healthy" != "true" ] && [ $attempt -lt $max_attempts ]; do
    ((attempt++))
    kafka_status=$(docker inspect --format '{{.State.Health.Status}}' loggix-kafka)
    zookeeper_status=$(docker inspect --format '{{.State.Health.Status}}' loggix-zookeeper)
    
    if [ "$kafka_status" = "healthy" ] && [ "$zookeeper_status" = "healthy" ]; then
        healthy=true
    else
        echo "Waiting for services to be healthy (attempt $attempt/$max_attempts)..."
        sleep 5
    fi
done

if [ "$healthy" != "true" ]; then
    echo "Services failed to become healthy within timeout"
    docker-compose down
    exit 1
fi

echo "Services are healthy, running tests..."

# Run the integration tests
cargo test --features integration-tests -- --test-threads=1

# Cleanup
echo "Cleaning up containers..."
docker-compose down
