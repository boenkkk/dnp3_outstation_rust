use crate::scheduler::run_scheduler;
use dnp3::outstation::OutstationHandle;
use std::sync::Arc;

pub async fn run_outstation(
    mut outstation: OutstationHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    outstation.enable().await?;

    // Wrap the `OutstationHandle` in an `Arc` to share ownership
    let outstation_arc = Arc::new(outstation);

    // Pass the `Arc` to `run_scheduler`
    run_scheduler(outstation_arc);
    loop {}
}
