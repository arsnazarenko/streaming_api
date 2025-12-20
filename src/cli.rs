use clap::Parser;

/// Rust implementation of streaming API.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Kafka brokers addresses
    #[arg(long, short, required = true)]
    pub kafka_brokers: String,

    /// WebSocket server address
    #[arg(long, short, required = true)]
    pub ws_address: String,
}

pub fn parse() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse() {
        // Test with mock args
        let args = Args::parse_from([
            "test",
            "--kafka-brokers",
            "localhost:9092",
            "--ws-address",
            "localhost:8080",
        ]);
        assert_eq!(args.kafka_brokers, "localhost:9092");
        assert_eq!(args.ws_address, "localhost:8080");
    }
}
