use std::time::Duration;
use dnp3::decode::{AppDecodeLevel, LinkDecodeLevel, PhysDecodeLevel, TransportDecodeLevel};
use dnp3::link::*;
use dnp3::outstation::*;
use dnp3::outstation::database::EventBufferConfig;

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
    let max_event_buffer = 10;
    EventBufferConfig::new(
        max_event_buffer, // binary
        max_event_buffer, // double-bit binary
        max_event_buffer, // binary output status
        max_event_buffer,  // counter
        max_event_buffer,  // frozen counter
        max_event_buffer,  // analog
        max_event_buffer,  // analog output status
        max_event_buffer,  // octet string
    )
}
