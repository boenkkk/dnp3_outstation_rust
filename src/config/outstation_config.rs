use crate::event_buffer::get_event_buffer_config;
use dnp3::decode::{AppDecodeLevel, LinkDecodeLevel, PhysDecodeLevel, TransportDecodeLevel};
use dnp3::link::EndpointAddress;
use dnp3::outstation::OutstationConfig;
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

    // Set decode levels from environment variables
    config.decode_level.application =
        get_decode_level("DNP3_OUTSTATION_LEVEL_APPLICATION", |s| match s {
            "Nothing" => AppDecodeLevel::Nothing,
            "Header" => AppDecodeLevel::Header,
            "ObjectHeaders" => AppDecodeLevel::ObjectHeaders,
            "ObjectValues" => AppDecodeLevel::ObjectValues,
            _ => AppDecodeLevel::Nothing,
        });

    config.decode_level.transport =
        get_decode_level("DNP3_OUTSTATION_LEVEL_TRANSPORT", |s| match s {
            "Nothing" => TransportDecodeLevel::Nothing,
            "Header" => TransportDecodeLevel::Header,
            "Payload" => TransportDecodeLevel::Payload,
            _ => TransportDecodeLevel::Nothing,
        });

    config.decode_level.link = get_decode_level("DNP3_OUTSTATION_LEVEL_LINK", |s| match s {
        "Nothing" => LinkDecodeLevel::Nothing,
        "Header" => LinkDecodeLevel::Header,
        "Payload" => LinkDecodeLevel::Payload,
        _ => LinkDecodeLevel::Nothing,
    });

    config.decode_level.physical =
        get_decode_level("DNP3_OUTSTATION_LEVEL_PHYSICAL", |s| match s {
            "Nothing" => PhysDecodeLevel::Nothing,
            "Length" => PhysDecodeLevel::Length,
            "Data" => PhysDecodeLevel::Data,
            _ => PhysDecodeLevel::Nothing,
        });

    config
}

// Helper function to reduce repetition when setting decode levels
fn get_decode_level<T, F>(env_var: &str, parser: F) -> T
where
    F: FnOnce(&str) -> T,
{
    let value = env::var(env_var).unwrap_or_else(|_| "Nothing".to_string());
    parser(&value)
}
