use dnp3::decode::AppDecodeLevel;
use dnp3::link::*;
use dnp3::outstation::*;
use dnp3::outstation::database::EventBufferConfig;

pub fn get_outstation_config() -> OutstationConfig {
    let outstation_address = 1;
    let master_address = 2;
    let mut config = OutstationConfig::new(
        EndpointAddress::try_new(outstation_address).unwrap(),
        EndpointAddress::try_new(master_address).unwrap(),
        get_event_buffer_config(),
    );
    config.class_zero.octet_string = true;
    config.decode_level.application = AppDecodeLevel::ObjectValues;
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
