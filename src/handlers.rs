// src/handlers.rs
use crate::time::get_current_time;
use dnp3::app::control::*;
use dnp3::app::measurement::*;
use dnp3::outstation::database::*;
use dnp3::outstation::*;

pub struct ExampleControlHandler;

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
