macro_rules! gd_log {
    ( $( $t:tt )* ) => {
        use godot::prelude::ToVariant;
        godot::engine::utilities::print(
                format!( $( $t )* ).to_variant(),
                &[]
        );
    }
}

fn simple_log(text: &str) {
    gd_log!("{}", text);
}

pub fn set_logger() {
    ds_lib::log::set_log_fn(Box::new(|text: &str| {
        simple_log(text);
    }));
}
