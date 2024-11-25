use crate::event_buffer::get_event_buffer_config;
use dnp3::decode::{AppDecodeLevel, LinkDecodeLevel, PhysDecodeLevel, TransportDecodeLevel};
use dnp3::link::EndpointAddress;
use dnp3::outstation::OutstationConfig;
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
