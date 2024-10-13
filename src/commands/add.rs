use crate::utils::assertions::assert_equal_length_vectors;
use crate::utils::process_pipeline;
use crate::VecPlugin;
use itertools::Itertools;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
#[cfg(test)]
use nu_plugin_test_support::PluginTest;
#[cfg(test)]
use nu_protocol::ShellError;
use nu_protocol::{
    Category, Example, IntoValue, LabeledError, PipelineData, Signature, Span,
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
        "vec add"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec add")
            .input_output_types(vec![(Type::List(Box::new(Type::Number)), Type::Number)])
            .allow_variants_without_examples(true)
            .required(
                "second_vector",
                SyntaxShape::List(Box::new(SyntaxShape::Number)),
                "The second vector to add to the vector in the pipeline.",
            )
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Returns the addition of two vectors, represented as lists."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "addition", "sum"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Calculate the cosine similarity between two vectors",
            example: "[1 2 3] | vec add [3 4 -5]",
            result: Some(Value::test_list(vec![
                Value::test_int(4),
                Value::test_int(6),
                Value::test_int(-2),
            ])),
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
        sum_vectors(
            vector_lhs,
            vector_rhs.as_slice(),
            pipeline_span,
            command_span,
        )
    })
}

pub fn sum_vectors(
    vector_lhs: &[Value],
    vector_rhs: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    if let Some(error) = assert_equal_length_vectors(vector_lhs, vector_rhs, pipeline_span, command_span) {
        return Err(error);
    }

    let vector_element_pairs = vector_lhs.iter().zip(vector_rhs);
    let output_values: Vec<Value> = vector_element_pairs
        .map(|(pipeline_value, arg_value)| {
            pipeline_value
                .add(command_span, arg_value, pipeline_span)
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
