# Devlog 03: Observability & Load Testing

## Context
Phase 2 was all about "measure before you optimize." Before I could tackle the performance bottlenecks I anticipated in Phase 1, I needed to instrument the system properly. This phase focused on structured logging, metrics collection, and establishing a baseline with load testing.

## Lessons Learned

### Structured Logging with `tracing`
I replaced basic logging with the `tracing` ecosystem (`tracing` + `tracing-subscriber`).
- **Benefit**: JSON-formatted logs make it easier to parse and analyze log data programmatically.
- **Request ID Correlation**: I added a request ID middleware using `tower_http::request_id::PropagateRequestIdLayer`. Every log entry now includes the request ID, making it easy to trace a single request through the entire system.
- **Insight**: Structured logging is a game-changer when debugging production issues. Being able to grep by request ID or search for specific events in JSON logs saves hours of troubleshooting.

### Prometheus Metrics
I integrated Prometheus to expose key metrics from the server.
- **Middleware**: I created a global metrics middleware that records HTTP request durations in a histogram (`http_requests_duration_seconds`).
- **SQLx Pool Metrics**: I also exposed internal connection pool stats (active connections, idle connections, waiters) to monitor database connection usage.
- **Endpoint**: All metrics are available at `/metrics` for Prometheus to scrape.

### Docker Compose Profiles
To keep the development environment modular, I introduced Docker Compose **profiles**.
- **Default**: Running `docker-compose up -d` only starts the Postgres database.
- **Observability**: `docker-compose --profile observability up -d` starts Prometheus and Grafana for monitoring.
- **Tools**: `docker-compose --profile tools run --rm k6 run /scripts/load_test.js` runs the k6 load test.
- **Benefit**: I can now start only what I need, avoiding resource waste when I'm just developing.

### Load Testing with k6
I set up k6 to run automated load tests against the server.
- **Script**: The load test fetches users in the `setup()` phase and then creates orders concurrently, expecting status codes 201 (created), 409 (conflict/sold out), or 404 (not found).
- **Initial Challenge**: The k6 container couldn't reach `localhost:3000` because it was running in a separate network. I had to switch to `host.docker.internal` to allow the container to access the host machine.

### Phase 2 Load Test Results
After completing the observability instrumentation, I ran a comprehensive 30-second load test with 10 concurrent users:

**Performance Metrics:**
- **Throughput**: ~88.3 RPS (Requests Per Second)
- **Total Requests**: 2,661 requests
- **Latency**:
    - **Average**: 11.86ms
    - **Median**: 7.64ms
    - **P90**: 16.94ms
    - **P95**: 26.96ms
    - **Max**: 344.17ms

**Success Metrics:**
- **Checks Passed**: 98.12% (2,610 out of 2,660)
- **Checks Failed**: 1.87% (50 out of 2,660)
- **HTTP Failures**: 98.08% marked as "failed" but this includes expected 409/404 responses for sold-out scenarios

**Analysis:**
- The system maintains consistent performance under the 100ms sleep constraint (~88 RPS ≈ 10 users / 0.11s).
- **Latency for successful requests** (201 responses) is significantly higher: avg=66.13ms, p(95)=296.86ms, indicating that database locking is the bottleneck.
- The high "failure" rate (98.08%) is expected behavior—it represents 409 (conflict/sold out) and 404 responses, not actual errors.
- The P95 latency of ~27ms for all requests shows reasonable performance, but successful writes can spike up to ~297ms, revealing the lock contention cost.
- This baseline confirms that pessimistic locking works correctly but will struggle under higher concurrency.

### Observability Stack Integration
I integrated Prometheus and Grafana into the Docker Compose setup.
- **Prometheus**: Scrapes the `/metrics` endpoint every 15 seconds.
- **Grafana**: Pre-configured with Prometheus as a data source and anonymous admin access for quick iteration.
- **Workflow**: I can now visualize request latencies, throughput, and connection pool usage in real-time.

## Observations
- **Structured logs** make debugging significantly easier, especially when combined with request IDs.
- **Metrics** give me objective data to identify bottlenecks and validate optimizations.
- **Load testing** is essential. Without it, I'd be optimizing blindly.
- The **Docker Compose profiles** pattern scales well for managing dev environments with multiple services.

## Next Steps
Now that I have observability in place, I'm ready to move to Phase 3: implementing admission control and exploring optimistic concurrency to improve throughput under high contention.
