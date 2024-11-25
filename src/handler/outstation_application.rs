use dnp3::app::attr::Attribute;
use dnp3::app::MaybeAsync;
use dnp3::outstation::OutstationApplication;

pub struct ExampleOutstationApplication;
impl OutstationApplication for ExampleOutstationApplication {
    fn support_write_analog_dead_bands(&mut self) -> bool {
        true
    }

    fn write_analog_dead_band(&mut self, index: u16, dead_band: f64) {
        tracing::info!("change analog dead-band {index} to {dead_band}");
    }

    fn write_device_attr(&mut self, attr: Attribute) -> MaybeAsync<bool> {
        tracing::info!("write device attribute: {:?}", attr);
        MaybeAsync::ready(true)
    }
}
