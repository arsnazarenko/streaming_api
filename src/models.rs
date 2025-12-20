#[derive(Debug, Clone)]
pub struct KafkaEvent {
    pub topic: String,
    pub payload: String,
}
