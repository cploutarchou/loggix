# Start Docker containers
Write-Host "Starting Kafka containers..."
docker-compose up -d

# Wait for services to be healthy
Write-Host "Waiting for services to be healthy..."
$maxAttempts = 30
$attempt = 0
$healthy = $false

while (-not $healthy -and $attempt -lt $maxAttempts) {
    $attempt++
    $kafkaStatus = docker inspect --format '{{.State.Health.Status}}' loggix-kafka
    $zookeeperStatus = docker inspect --format '{{.State.Health.Status}}' loggix-zookeeper
    
    if ($kafkaStatus -eq "healthy" -and $zookeeperStatus -eq "healthy") {
        $healthy = $true
    } else {
        Write-Host "Waiting for services to be healthy (attempt $attempt/$maxAttempts)..."
        Start-Sleep -Seconds 5
    }
}

if (-not $healthy) {
    Write-Host "Services failed to become healthy within timeout"
    docker-compose down
    exit 1
}

Write-Host "Services are healthy, running tests..."

# Run the integration tests
cargo test --features integration-tests -- --test-threads=1

# Cleanup
Write-Host "Cleaning up containers..."
docker-compose down
