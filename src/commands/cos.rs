use crate::commands::dot::compute_dot_product;
use crate::commands::magnitude::compute_magnitude;
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
        "vec cos"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec cos")
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
        "Returns the cosine of the angle between vectors, represented as lists."
    }

    fn extra_description(&self) -> &str {
        "This is often used to determine the similarity of two vectors."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "cosine", "angle", "similarity"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Calculate the cosine similarity between two vectors",
            example: "[1 2 3] | vec cos [3 4 -5]",
            result: Some(Value::test_float(-0.15118578920369088)),
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
        compute_vcos(
            vector_lhs,
            vector_rhs.as_slice(),
            pipeline_span,
            command_span,
        )
    })
}

pub fn compute_vcos(
    vector_lhs: &[Value],
    vector_rhs: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    if let Some(error) =
        assert_equal_length_vectors(vector_lhs, vector_rhs, pipeline_span, command_span)
    {
        return Err(error);
    }

    let dot_product = compute_dot_product(vector_lhs, vector_rhs, pipeline_span, command_span)?;

    let magnitude_lhs = compute_magnitude(vector_lhs, pipeline_span, command_span)?;
    let magnitude_rhs = compute_magnitude(vector_rhs, pipeline_span, command_span)?;
    let magnitude_product = magnitude_lhs.mul(command_span, &magnitude_rhs, command_span)?;

    let output = dot_product.div(command_span, &magnitude_product, command_span);

    output.map_err(LabeledError::from)
}

#[cfg(test)]
#[test]
fn test_examples() -> Result<(), ShellError> {
    PluginTest::new("nu_plugin_vec", VecPlugin.into())?.test_command_examples(&Command)
}
