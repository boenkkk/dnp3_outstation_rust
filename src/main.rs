//! Example master application
use dnp3::app::control::*;
use dnp3::app::measurement::*;
use dnp3::app::*;
use dnp3::decode::*;
use dnp3::link::*;
use dnp3::outstation::database::*;
use dnp3::outstation::*;

use dnp3::app::attr::{AttrProp, Attribute, StringAttr};
use dnp3::tcp::*;

/// example of using the outstation API asynchronously from within the Tokio runtime
///
/// The application takes a single command line argument specifying the desired transport
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Always run the TCP server, bypassing the transport switch statement
    run_tcp_server().await?;

    Ok(())
}

struct ExampleOutstationApplication;
impl OutstationApplication for ExampleOutstationApplication {
    fn support_write_analog_dead_bands(&mut self) -> bool {
        true
    }

    fn write_analog_dead_band(&mut self, index: u16, dead_band: f64) {
        tracing::info!("change analog dead-band {index} to {dead_band}");
    }

    fn write_device_attr(&mut self, attr: Attribute) -> MaybeAsync<bool> {
        tracing::info!("write device attribute: {:?}", attr);
        // Allow writing any attribute that has been defined as writable
        MaybeAsync::ready(true)
    }
}

struct ExampleOutstationInformation;
impl OutstationInformation for ExampleOutstationInformation {}

// ANCHOR: control_handler
struct ExampleControlHandler;
impl ControlHandler for ExampleControlHandler {}

impl ControlSupport<Group12Var1> for ExampleControlHandler {
    fn select(
        &mut self,
        control: Group12Var1,
        index: u16,
        _database: &mut DatabaseHandle,
    ) -> CommandStatus {
        if index < 10
            && (control.code.op_type == OpType::LatchOn || control.code.op_type == OpType::LatchOff)
        {
            CommandStatus::Success
        } else {
            CommandStatus::NotSupported
        }
    }

    fn operate(
        &mut self,
        control: Group12Var1,
        index: u16,
        _op_type: OperateType,
        database: &mut DatabaseHandle,
    ) -> CommandStatus {
        if index < 10
            && (control.code.op_type == OpType::LatchOn || control.code.op_type == OpType::LatchOff)
        {
            let status = control.code.op_type == OpType::LatchOn;
            database.transaction(|db| {
                db.update(
                    index,
                    &BinaryOutputStatus::new(status, Flags::ONLINE, get_current_time()),
                    UpdateOptions::detect_event(),
                );
            });
            CommandStatus::Success
        } else {
            CommandStatus::NotSupported
        }
    }
}

impl ExampleControlHandler {
    fn select_analog_output(&self, index: u16) -> CommandStatus {
        if index < 10 {
            CommandStatus::Success
        } else {
            CommandStatus::NotSupported
        }
    }

    fn operate_analog_output(
        &self,
        value: f64,
        index: u16,
        database: &mut DatabaseHandle,
    ) -> CommandStatus {
        if index < 10 {
            database.transaction(|db| {
                db.update(
                    index,
                    &AnalogOutputStatus::new(value, Flags::ONLINE, get_current_time()),
                    UpdateOptions::detect_event(),
                );
            });
            CommandStatus::Success
        } else {
            CommandStatus::NotSupported
        }
    }
}

impl ControlSupport<Group41Var1> for ExampleControlHandler {
    fn select(
        &mut self,
        _control: Group41Var1,
        index: u16,
        _database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.select_analog_output(index)
    }

    fn operate(
        &mut self,
        control: Group41Var1,
        index: u16,
        _op_type: OperateType,
        database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.operate_analog_output(control.value as f64, index, database)
    }
}

impl ControlSupport<Group41Var2> for ExampleControlHandler {
    fn select(
        &mut self,
        _control: Group41Var2,
        index: u16,
        _database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.select_analog_output(index)
    }

    fn operate(
        &mut self,
        control: Group41Var2,
        index: u16,
        _op_type: OperateType,
        database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.operate_analog_output(control.value as f64, index, database)
    }
}

impl ControlSupport<Group41Var3> for ExampleControlHandler {
    fn select(
        &mut self,
        _control: Group41Var3,
        index: u16,
        _database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.select_analog_output(index)
    }

    fn operate(
        &mut self,
        control: Group41Var3,
        index: u16,
        _op_type: OperateType,
        database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.operate_analog_output(control.value as f64, index, database)
    }
}

impl ControlSupport<Group41Var4> for ExampleControlHandler {
    fn select(
        &mut self,
        _control: Group41Var4,
        index: u16,
        _database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.select_analog_output(index)
    }

    fn operate(
        &mut self,
        control: Group41Var4,
        index: u16,
        _op_type: OperateType,
        database: &mut DatabaseHandle,
    ) -> CommandStatus {
        self.operate_analog_output(control.value, index, database)
    }
}
// ANCHOR_END: control_handler

async fn run_tcp_server() -> Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: create_tcp_server
    let server = Server::new_tcp_server(LinkErrorMode::Close, "0.0.0.0:777".parse()?);
    // ANCHOR_END: create_tcp_server

    run_server(server).await
}

async fn run_server(mut server: Server) -> Result<(), Box<dyn std::error::Error>> {
    // ANCHOR: tcp_server_spawn_outstation
    let outstation = server.add_outstation(
        get_outstation_config(),
        Box::new(ExampleOutstationApplication),
        Box::new(ExampleOutstationInformation),
        Box::new(ExampleControlHandler),
        NullListener::create(),
        AddressFilter::Any,
    )?;
    // ANCHOR_END: tcp_server_spawn_outstation

    // set up the outstation's database before we spawn it
    // ANCHOR: database_init
    outstation.transaction(|db| {
        // initialize 10 points of each type
        for i in 0..10 {
            db.add(
                i,
                Some(EventClass::Class1),
                // you can explicitly specify the configuration for each point ...
                BinaryInputConfig {
                    s_var: StaticBinaryInputVariation::Group1Var1,
                    e_var: EventBinaryInputVariation::Group2Var2,
                },
            );
            db.add(
                i,
                Some(EventClass::Class1),
                // ... or just use the defaults
                DoubleBitBinaryInputConfig::default(),
            );
            db.add(
                i,
                Some(EventClass::Class1),
                BinaryOutputStatusConfig::default(),
            );
            db.add(i, Some(EventClass::Class1), CounterConfig::default());
            db.add(i, Some(EventClass::Class1), FrozenCounterConfig::default());
            db.add(
                i,
                Some(EventClass::Class1),
                AnalogInputConfig {
                    s_var: StaticAnalogInputVariation::Group30Var1,
                    e_var: EventAnalogInputVariation::Group32Var1,
                    deadband: 0.0,
                },
            );
            db.add(
                i,
                Some(EventClass::Class1),
                AnalogOutputStatusConfig::default(),
            );
            db.add(i, Some(EventClass::Class1), OctetStringConfig);
        }

        // define device attributes made available to the master
        let _ = db.define_attr(
            AttrProp::default(),
            StringAttr::DeviceManufacturersName.with_value("Step Function I/O"),
        );
        let _ = db.define_attr(
            AttrProp::writable(),
            StringAttr::UserAssignedLocation.with_value("Bend, OR"),
        );
    });
    // ANCHOR_END: database_init

    // ANCHOR: server_bind
    // dropping the ServerHandle shuts down the server and outstation(s)
    let _server_handle = server.bind().await?;
    // ANCHOR_END: server_bind

    run_outstation(outstation).await
}

// run the same logic regardless of the transport type
async fn run_outstation(
    mut outstation: OutstationHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    outstation.enable().await?;
    loop {}
}

fn get_current_time() -> Time {
    let epoch_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    Time::Synchronized(Timestamp::new(epoch_time.as_millis() as u64))
}

fn get_outstation_config() -> OutstationConfig {
    // ANCHOR: outstation_config
    // create an outstation configuration with default values
    let mut config = OutstationConfig::new(
        // outstation address
        EndpointAddress::try_new(1).unwrap(),
        // master address
        EndpointAddress::try_new(2).unwrap(),
        get_event_buffer_config(),
    );
    config.class_zero.octet_string = true;

    // override the default decoding
    config.decode_level.application = AppDecodeLevel::ObjectValues;
    // ANCHOR_END: outstation_config
    config
}

// ANCHOR: event_buffer_config
fn get_event_buffer_config() -> EventBufferConfig {
    EventBufferConfig::new(
        10, // binary
        10, // double-bit binary
        10, // binary output status
        5,  // counter
        5,  // frozen counter
        5,  // analog
        5,  // analog output status
        3,  // octet string
    )
}
// ANCHOR_END: event_buffer_config
