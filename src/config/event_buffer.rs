use dnp3::outstation::database::EventBufferConfig;
use std::env;

pub fn get_event_buffer_config() -> EventBufferConfig {
    EventBufferConfig::new(
        env::var("DNP3_BINARY_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // BINARY
        env::var("DNP3_DOUBLE_BIT_BINARY_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // DOUBLE_BIT_BINARY
        env::var("DNP3_BINARY_OUTPUT_STATUS_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // BINARY_OUTPUT_STATUS
        env::var("DNP3_COUNTER_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // COUNTER
        env::var("DNP3_FROZEN_COUNTER_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // FROZEN_COUNTER
        env::var("DNP3_ANALOG_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // ANALOG
        env::var("DNP3_ANALOG_OUTPUT_STATUS_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // ANALOG_OUTPUT_STATUS
        env::var("DNP3_OCTET_STRING_MAX_EVENT_BUFFER")
            .unwrap()
            .parse()
            .unwrap(), // OCTET_STRING
    )
}
