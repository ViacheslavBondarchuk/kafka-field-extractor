use rdkafka::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use rdkafka::util::current_time_millis;

use crate::model::Config;

pub struct StreamKafkaConsumer {
    stream_consumer: StreamConsumer,
}

impl StreamKafkaConsumer {
    pub fn new(config: &Config) -> Self {
        let mut client_config: ClientConfig = ClientConfig::new();
        &client_config.set("bootstrap.servers", &config.bootstrap_servers);
        &client_config.set("auto.offset.reset", "earliest");
        &client_config.set("group.id", format!("kafka-field-extractor-{}", current_time_millis()));
        &client_config.set("schema.registry.url", &config.schema_registry);

        if let Some(security_protocol) = &config.security_protocol {
            &client_config.set("security.protocol", security_protocol);
        }
        if let Some(inter_broker_protocol) = &config.security_inter_broker_protocol {
            &client_config.set("ssl.endpoint.identification.algorithm", inter_broker_protocol);
        }
        if let Some(ssl_key_location) = &config.ssl_key_location {
            &client_config.set("ssl.key.location", ssl_key_location);
        }
        if let Some(ssl_key_password) = &config.ssl_key_password {
            &client_config.set("ssl.key.password", ssl_key_password);
        }
        if let Some(ssl_certificate_location) = &config.ssl_certificate_location {
            &client_config.set("ssl.certificate.location", ssl_certificate_location);
        }
        if let Some(ssl_ca_location) = &config.ssl_ca_location {
            &client_config.set("ssl.ca.location", ssl_ca_location);
        }
        if let Some(ssl_enabled_protocols) = &config.ssl_enabled_protocols {
            &client_config.set("ssl.enabled.protocols", ssl_enabled_protocols);
        }

        let stream_consumer: StreamConsumer = client_config.create().expect("Stream kafka consumer creation error");
        Self { stream_consumer }
    }
}