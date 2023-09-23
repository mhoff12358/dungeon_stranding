use godot::prelude::godot_print;

fn simple_log(text: &str) {
    godot_print!("{}", text);
}

pub fn set_logger() {
    ds_lib::log::set_log_fn(Box::new(|text: &str| {
        simple_log(text);
    }));
}
