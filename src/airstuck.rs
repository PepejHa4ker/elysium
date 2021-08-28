use crate::command::Command;
use crate::input::{Button, Input};

// https://stackoverflow.com/questions/12596695/why-does-a-float-variable-stop-incrementing-at-16777216-in-c
pub const TWO24: i32 = 2_i32.pow(24);

pub fn create_move<'a>(command: &'a mut Command, input: &'a Input, button: Button) {
    let mut logger = Logger::new();

    writeln!(logger, "airstuck!");
    writeln!(logger, "command = {:?}", command);
    writeln!(logger, "input = {:?}", input);
    writeln!(logger, "button = {:?}", button);

    if command.in_attack() || command.in_attack2() {
        return;
    }

    if input.is_button_down(button) {
        command.set_tick_count(TWO24);
    }
}
