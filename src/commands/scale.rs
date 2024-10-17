use crate::utils::assertions::assert_equal_length_vectors;
use crate::utils::process_pipeline;
use crate::VecPlugin;
use itertools::Itertools;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
#[cfg(test)]
use nu_plugin_test_support::PluginTest;
use nu_protocol::{
    Category, Example, IntoValue, LabeledError, PipelineData, ShellError, Signature, Span,
    SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct Command;

impl PluginCommand for Command {
    type Plugin = VecPlugin;

    fn name(&self) -> &str {
        "vec scale"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec scale")
            .input_output_types(vec![(
                Type::List(Box::new(Type::Number)),
                Type::List(Box::new(Type::Number)),
            )])
            .allow_variants_without_examples(true)
            .required(
                "scaling_factor",
                SyntaxShape::OneOf(vec![SyntaxShape::Number, SyntaxShape::List(Box::new(SyntaxShape::Number))]),
                "The factor to multiply the vector by (either a number or same-size vector can be used)",
            )
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Returns the scaled vector, represented as list."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "scaling", "multplication"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Scale a vector uniformly",
                example: "[1 2 3] | vec scale 10",
                result: Some(Value::test_list(vec![
                    Value::test_int(10),
                    Value::test_int(20),
                    Value::test_int(30),
                ])),
            },
            Example {
                description: "Scale a vector with different scales per dimension",
                example: "[1 2 3] | vec scale [10 100 1000]",
                result: Some(Value::test_list(vec![
                    Value::test_int(10),
                    Value::test_int(200),
                    Value::test_int(3000),
                ])),
            },
        ]
    }

    fn run(
        &self,
        _plugin: &VecPlugin,
        _engine_state: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        process_pipeline(
            call,
            input,
            |vector_subject, pipeline_span, command_span| {
                let argument = call.positional.first().unwrap();

                if let Value::List { ref vals, .. } = argument {
                    let vector_factors = vals.clone();
                    return scale_vector_stretching(
                        vector_subject,
                        vector_factors.as_slice(),
                        pipeline_span,
                        command_span,
                    );
                } else if let Value::Int { .. } | Value::Float { .. } = argument {
                    let factor = argument.clone();
                    return scale_vector_uniformly(
                        vector_subject,
                        factor,
                        pipeline_span,
                        command_span,
                    );
                } else {
                    Err(LabeledError::from(ShellError::TypeMismatch {
                        err_message: "This should not be reachable, file a bug report!".to_string(),
                        span: argument.span(),
                    }))
                }
            },
        )
    }
}

/// Scales a vector, applying the same scaling to each component of the vector.
///
/// This causes the different components of the vector to stay in proportion to each other,
/// effectively only the magnitude of the vector is adjusted.
///
/// # Arguments
/// * `vector` - The vector to scale
/// * `factor` - The multiplication factor to apply to the vector
/// * `pipeline_span` - The span belonging to the vector
/// * `command_span` - The span belonging to the command which initiated this action
///
/// ---
///
/// See also: [scale_vector_stretching]
#[allow(clippy::result_large_err)]
pub fn scale_vector_uniformly(
    vector: &[Value],
    factor: Value,
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    let output_values: Vec<Value> = vector
        .iter()
        .map(|pipeline_value| {
            pipeline_value
                .mul(command_span, &factor, pipeline_span)
                .unwrap_or(Value::float(0f64, command_span))
        })
        .collect_vec();
    let output = output_values.into_value(command_span);

    Ok(output)
}

/// Scales a vector, allowing a different scaling to be applied per vector component.
///
/// This is where the term "stretch" originates, as the proportions between the original vector
/// components is warped when those components are not all equal.
///
/// # Arguments
/// * `vector` - The vector to scale
/// * `factors` - The multiplication factor to apply, on for each component of the vector
/// * `pipeline_span` - The span belonging to the vector
/// * `command_span` - The span belonging to the command which initiated this action
///
/// ---
#[allow(clippy::result_large_err)]
pub fn scale_vector_stretching(
    vector: &[Value],
    factors: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    if let Some(error) =
        assert_equal_length_vectors(vector, factors, pipeline_span, command_span)
    {
        return Err(error);
    }

    let vector_element_pairs = vector.iter().zip(factors);
    let output_values: Vec<Value> = vector_element_pairs
        .map(|(pipeline_value, arg_value)| {
            pipeline_value
                .mul(command_span, arg_value, pipeline_span)
                .unwrap_or(Value::float(0f64, command_span))
        })
        .collect_vec();
    let output = output_values.into_value(command_span);

    Ok(output)
}

#[cfg(test)]
#[test]
fn test_examples() -> Result<(), ShellError> {
    PluginTest::new("nu_plugin_vec", VecPlugin.into())?.test_command_examples(&Command)
}
