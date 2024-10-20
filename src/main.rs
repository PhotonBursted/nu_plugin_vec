use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_vec::VecPlugin;

fn main() {
    serve_plugin(&VecPlugin, MsgPackSerializer)
}
