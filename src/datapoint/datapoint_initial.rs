#[path = "analog_input.rs"]
mod analog_input;
#[path = "analog_output.rs"]
mod analog_output;
#[path = "binary_input.rs"]
mod binary_input;
#[path = "binary_output.rs"]
mod binary_output;
#[path = "counter.rs"]
mod counter;
#[path = "double_bit_binary_input.rs"]
mod double_bit_binary_input;
#[path = "frozen_counter.rs"]
mod frozen_counter;

use analog_input::initial_analog_input;
use analog_output::initial_analog_output;
use binary_input::initial_binary_input;
use binary_output::initial_binary_output;
use counter::initial_counter;
use double_bit_binary_input::initial_double_bit_binary_input;
use frozen_counter::initial_frozen_counter;

use dnp3::app::attr::{AttrProp, StringAttr};
use dnp3::outstation::database::{Add, EventClass, OctetStringConfig};
use dnp3::outstation::OutstationHandle;

pub fn initialize_database(outstation: &OutstationHandle) {
    outstation.transaction(|db| {
        let _ = db.define_attr(
            AttrProp::default(),
            StringAttr::DeviceManufacturersName.with_value("Step Function I/O"),
        );
        let _ = db.define_attr(
            AttrProp::writable(),
            StringAttr::UserAssignedLocation.with_value("Bend, OR"),
        );
        db.add(0, Some(EventClass::Class1), OctetStringConfig);

        initial_binary_input(db);
        initial_binary_output(db);
        initial_analog_input(db);
        initial_analog_output(db);
        initial_double_bit_binary_input(db);
        initial_counter(db);
        initial_frozen_counter(db);
    });
}
