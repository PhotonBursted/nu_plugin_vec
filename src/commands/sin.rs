use crate::commands::cos::compute_vcos;
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

struct Arguments {
    vector_rhs: Vec<f64>,
}

#[derive(Clone)]
pub struct Command;

impl PluginCommand for Command {
    type Plugin = VecPlugin;

    fn name(&self) -> &str {
        "vec sin"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec sin")
            .input_output_types(vec![(Type::List(Box::new(Type::Number)), Type::Number)])
            .allow_variants_without_examples(true)
            .required(
                "second_vector",
                SyntaxShape::List(Box::new(SyntaxShape::Number)),
                "The second vector to compare to the vector in the pipeline.",
            )
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Returns the sine of the angle between vectors, represented as lists."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "sine", "angle"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Calculate the sine of the angle between two vectors",
            example: "[1 2 3] | vec sin [3 4 -5]",
            result: Some(Value::test_float(0.9885053652574968)),
        }]
    }

    fn run(
        &self,
        _plugin: &VecPlugin,
        _engine_state: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let args = Arguments {
            vector_rhs: call.req::<Vec<f64>>(0)?,
        };

        operate(call, input, args)
    }
}

fn operate(
    call: &EvaluatedCall,
    input: PipelineData,
    args: Arguments,
) -> Result<PipelineData, LabeledError> {
    let head = call.head;

    let vector_rhs = args
        .vector_rhs
        .iter()
        .map(|float| float.into_value(head))
        .collect_vec();

    process_pipeline(call, input, |vector_lhs, pipeline_span, command_span| {
        compute_vsin(
            vector_lhs,
            vector_rhs.as_slice(),
            pipeline_span,
            command_span,
        )
    })
}

pub fn compute_vsin(
    vector_lhs: &[Value],
    vector_rhs: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    if let Some(error) = assert_equal_length_vectors(vector_lhs, vector_rhs, pipeline_span, command_span) {
        return Err(error);
    }

    let cosine = compute_vcos(vector_lhs, vector_rhs, pipeline_span, command_span)?;
    let cosine_squared = cosine.mul(command_span, &cosine, command_span)?;
    let output_squared =
        Value::int(1, command_span).sub(command_span, &cosine_squared, command_span)?;
    let output = output_squared
        .coerce_float()
        .map(|float| float.sqrt().into_value(command_span));

    output.map_err(LabeledError::from)
}

#[cfg(test)]
#[test]
fn test_examples() -> Result<(), ShellError> {
    PluginTest::new("nu_plugin_vec", VecPlugin.into())?.test_command_examples(&Command)
}
