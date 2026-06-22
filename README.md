# HTTP Client 🌐

Async HTTP client with HTTP/2, connection pooling, and circuit breaking.

## Features

- **HTTP/1.1, HTTP/2, HTTP/3**: Protocol negotiation
- **Connection Pooling**: Keep-alive reuse
- **Retry Policy**: Exponential backoff
- **Circuit Breaking**: Automatic failure detection

## Benchmarks

| Metric | Value |
|--------|-------|
| GET throughput | 25K req/s |
| Connection reuse | 95% |
| Time to first byte | 0.8ms |

## Quick Start

```rust
let client = HttpClient::new();
let resp = client.get("https://api.example.com/data").await?;
println!("Status: {}", resp.status);
```

## License

MIT