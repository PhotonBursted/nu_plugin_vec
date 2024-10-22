use nu_plugin_test_support::PluginTest;
use nu_plugin_vec::commands::Sub as Command;
use nu_plugin_vec::{nu, test_examples, VecPlugin};
use nu_protocol::ShellError;

test_examples!();

#[test]
fn errors_when_left_vector_is_shorter() {
    let result = nu!("\
                let a = [1 2];\
                let b = [1 2 3 4 5];\
                \
                $a | vec sub $b");

    assert!(result.is_err());
}

#[test]
fn errors_when_right_vector_is_shorter() {
    let result = nu!("\
                let a = [1 2 3 4 5];\
                let b = [1 2 3];\
                \
                $a | vec sub $b");

    assert!(result.is_err());
}
