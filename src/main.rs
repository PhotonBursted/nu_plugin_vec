mod commands;
mod utils;

use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, PluginCommand};

pub struct VecPlugin;

impl Plugin for VecPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin=Self>>> {
        vec![
            Box::new(commands::Add),
            Box::new(commands::Cosine),
            Box::new(commands::Dot),
            Box::new(commands::Magnitude),
            Box::new(commands::Normalize),
            Box::new(commands::Sine),
            Box::new(commands::SqNorm),
            Box::new(commands::Sub),
        ]
    }
}

fn main() {
    serve_plugin(&VecPlugin, MsgPackSerializer)
}
