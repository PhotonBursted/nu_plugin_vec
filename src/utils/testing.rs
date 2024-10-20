/// Generates a vector of Nushell values.
///
/// # Usage
/// `vec_of_values![<type>; <value1>, value2, ..., valuen]`, where:
/// * `<type>` - The path to a Value type *(e.g. `Value::int`)*
/// * `<value>` - The things to include in the list, and convert to values
#[macro_export]
macro_rules! vec_of_values {
    ( $( $val_type:path )?; $( $x:expr ),+ ) => {
        {
            vec![$( $x, )*]
                .into_iter()
                .map(|value| $($val_type)?(value, Span::test_data()))
                .collect_vec()
        }
    };
}

/// Generates a test which tests the examples belonging to a given command.
///
/// The command under test should be available in scope as `Command`.
#[macro_export]
macro_rules! test_examples {
    () => {
        #[test]
        fn test_examples() -> Result<(), ShellError> {
            PluginTest::new("nu_plugin_vec", VecPlugin.into())
                .unwrap()
                .test_command_examples(&Command)
        }
    };
}

/// Evaluates a literal by passing it into a virtual Nushell session.
///
/// The output of the script is returned as a `Result`, which either contains the `PipelineData`
/// when the script completed successfully, or the `ShellError` when something went wrong while
/// executing.
#[macro_export]
macro_rules! nu {
    ( $( $x:literal )? ) => {
        PluginTest::new("nu_plugin_vec", VecPlugin.into()).unwrap().eval($( $x )?)
    }
}
