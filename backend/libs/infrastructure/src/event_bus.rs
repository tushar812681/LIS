use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::util::Timeout;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error, warn};
use uuid::Uuid;

use common::error::{Error, Result};

/// Domain event that gets published to Kafka
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: String,
    pub aggregate_type: String,
    pub payload: serde_json::Value,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub organization_id: String,
    pub user_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
}

impl DomainEvent {
    pub fn new(
        event_type: String,
        aggregate_id: String,
        aggregate_type: String,
        payload: serde_json::Value,
        organization_id: String,
        user_id: Option<String>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            event_type,
            aggregate_id,
            aggregate_type,
            payload,
            metadata: EventMetadata {
                organization_id,
                user_id,
                timestamp: chrono::Utc::now(),
                correlation_id: None,
                causation_id: None,
            },
        }
    }
}

/// Event bus for publishing and consuming domain events
#[derive(Clone)]
pub struct EventBus {
    producer: FutureProducer,
    brokers: String,
}

impl EventBus {
    pub fn new(brokers: &str) -> Result<Self> {
        info!("Connecting to Kafka brokers: {}", brokers);

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "30000")
            .set("compression.type", "snappy")
            .set("batch.size", "16384")
            .set("linger.ms", "10")
            .set("acks", "all")
            .set("retries", "3")
            .create()
            .map_err(|e| {
                error!("Failed to create Kafka producer: {}", e);
                Error::Kafka(format!("Producer creation failed: {}", e))
            })?;

        info!("Successfully connected to Kafka");

        Ok(Self {
            producer,
            brokers: brokers.to_string(),
        })
    }

    pub async fn publish(&self, topic: &str, event: &DomainEvent) -> Result<()> {
        let key = event.aggregate_id.clone();
        let payload = serde_json::to_string(event)
            .map_err(|e| Error::Serialization(e))?;

        let headers = OwnedHeaders::new()
            .insert(Header {
                key: "event_type",
                value: Some(event.event_type.as_bytes()),
            })
            .insert(Header {
                key: "event_id",
                value: Some(event.event_id.to_string().as_bytes()),
            })
            .insert(Header {
                key: "organization_id",
                value: Some(event.metadata.organization_id.as_bytes()),
            });

        let record = FutureRecord::to(topic)
            .key(&key)
            .payload(&payload)
            .headers(headers);

        self.producer
            .send(record, Timeout::After(Duration::from_secs(10)))
            .await
            .map_err(|(err, _)| {
                error!("Failed to publish event to Kafka: {}", err);
                Error::Kafka(format!("Event publish failed: {}", err))
            })?;

        info!(
            "Published event {} to topic {}",
            event.event_type, topic
        );

        Ok(())
    }

    pub async fn publish_batch(&self, topic: &str, events: Vec<DomainEvent>) -> Result<()> {
        for event in events {
            self.publish(topic, &event).await?;
        }
        Ok(())
    }
}

/// Event publisher trait for dependency injection
pub trait EventPublisher: Send + Sync {
    fn publish(&self, topic: &str, event: &DomainEvent) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl EventPublisher for EventBus {
    async fn publish(&self, topic: &str, event: &DomainEvent) -> Result<()> {
        EventBus::publish(self, topic, event).await
    }
}

/// Event consumer for subscribing to topics
pub struct EventConsumer {
    consumer: StreamConsumer,
}

impl EventConsumer {
    pub fn new(brokers: &str, group_id: &str, topics: &[&str]) -> Result<Self> {
        info!("Creating Kafka consumer for group: {}", group_id);

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.auto.commit", "true")
            .set("auto.commit.interval.ms", "5000")
            .set("session.timeout.ms", "30000")
            .set("enable.partition.eof", "false")
            .set("auto.offset.reset", "earliest")
            .create()
            .map_err(|e| {
                error!("Failed to create Kafka consumer: {}", e);
                Error::Kafka(format!("Consumer creation failed: {}", e))
            })?;

        consumer
            .subscribe(topics)
            .map_err(|e| {
                error!("Failed to subscribe to topics: {}", e);
                Error::Kafka(format!("Subscription failed: {}", e))
            })?;

        info!("Successfully subscribed to topics: {:?}", topics);

        Ok(Self { consumer })
    }

    // In production, would implement async stream consumption
    // pub async fn consume_stream(&self) -> impl Stream<Item = Result<DomainEvent>>
}

// Common event types as constants
pub mod events {
    // Patient events
    pub const PATIENT_CREATED: &str = "patient.created";
    pub const PATIENT_UPDATED: &str = "patient.updated";
    pub const PATIENT_DELETED: &str = "patient.deleted";

    // Sample events
    pub const SAMPLE_COLLECTED: &str = "sample.collected";
    pub const SAMPLE_RECEIVED: &str = "sample.received";
    pub const SAMPLE_REJECTED: &str = "sample.rejected";
    pub const SAMPLE_ROUTED: &str = "sample.routed";

    // Order events
    pub const ORDER_CREATED: &str = "order.created";
    pub const ORDER_CONFIRMED: &str = "order.confirmed";
    pub const ORDER_CANCELLED: &str = "order.cancelled";
    pub const ORDER_COMPLETED: &str = "order.completed";

    // Result events
    pub const RESULT_ENTERED: &str = "result.entered";
    pub const RESULT_VERIFIED: &str = "result.verified";
    pub const RESULT_AMENDED: &str = "result.amended";
    pub const CRITICAL_VALUE_DETECTED: &str = "result.critical_value_detected";

    // Report events
    pub const REPORT_GENERATED: &str = "report.generated";
    pub const REPORT_SIGNED: &str = "report.signed";
    pub const REPORT_DELIVERED: &str = "report.delivered";

    // Payment events
    pub const PAYMENT_RECEIVED: &str = "payment.received";
    pub const PAYMENT_FAILED: &str = "payment.failed";
}

// Topic names
pub mod topics {
    pub const PATIENT_EVENTS: &str = "lis.patient.events";
    pub const SAMPLE_EVENTS: &str = "lis.sample.events";
    pub const ORDER_EVENTS: &str = "lis.order.events";
    pub const RESULT_EVENTS: &str = "lis.result.events";
    pub const REPORT_EVENTS: &str = "lis.report.events";
    pub const BILLING_EVENTS: &str = "lis.billing.events";
    pub const NOTIFICATION_EVENTS: &str = "lis.notification.events";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_event_creation() {
        let event = DomainEvent::new(
            "patient.created".to_string(),
            Uuid::new_v4().to_string(),
            "Patient".to_string(),
            serde_json::json!({"name": "John Doe"}),
            Uuid::new_v4().to_string(),
            Some(Uuid::new_v4().to_string()),
        );

        assert_eq!(event.event_type, "patient.created");
        assert_eq!(event.aggregate_type, "Patient");
    }
}
