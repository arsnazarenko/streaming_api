# STREAMING_API
Usage:

```bash
Rust implementation of streaming API

Usage: streaming_api --kafka-brokers <KAFKA_BROKERS> --ws-address <WS_ADDRESS>

Options:
  -k, --kafka-brokers <KAFKA_BROKERS>  Kafka brokers addresses
  -w, --ws-address <WS_ADDRESS>        WebSocket server address
  -h, --help                           Print help
  -V, --version                        Print version
```

# STREAMING_CLIENT
Simple webpage thats connect to localhost:8181 and read messages via webdocket;
Usage:
1. Run rust app
2. Open in browser ./static/streaming_client.html
