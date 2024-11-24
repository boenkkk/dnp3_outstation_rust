use dnp3::decode::{AppDecodeLevel, LinkDecodeLevel, PhysDecodeLevel, TransportDecodeLevel};
use dnp3::link::*;
use dnp3::outstation::database::EventBufferConfig;
use dnp3::outstation::*;
use std::env;
use std::time::Duration;

pub fn get_outstation_config(outstation_address: u16, master_address: u16) -> OutstationConfig {
    let mut config = OutstationConfig::new(
        EndpointAddress::try_new(outstation_address).unwrap(),
        EndpointAddress::try_new(master_address).unwrap(),
        get_event_buffer_config(),
    );
    config.class_zero.octet_string = true;
    config.keep_alive_timeout = Some(Duration::from_secs(10));
    config.decode_level.application = AppDecodeLevel::ObjectValues;
    config.decode_level.transport = TransportDecodeLevel::Nothing;
    config.decode_level.link = LinkDecodeLevel::Nothing;
    config.decode_level.physical = PhysDecodeLevel::Nothing;
    config
}

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
