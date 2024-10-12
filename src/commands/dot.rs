use crate::utils::process_pipeline;
use crate::utils::reducers::sum;
use crate::VecPlugin;
use itertools::Itertools;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
#[cfg(test)]
use nu_plugin_test_support::PluginTest;
#[cfg(test)]
use nu_protocol::ShellError;
use nu_protocol::{
    Category, Example, IntoValue, LabeledError, PipelineData, Signature, Span, SyntaxShape, Type,
    Value,
};

struct Arguments {
    vector_rhs: Vec<f64>,
}

#[derive(Clone)]
pub struct Command;

impl PluginCommand for Command {
    type Plugin = VecPlugin;

    fn name(&self) -> &str {
        "vec dot"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec dot")
            .input_output_types(vec![(Type::List(Box::new(Type::Number)), Type::Number)])
            .allow_variants_without_examples(true)
            .required(
                "second_vector",
                SyntaxShape::List(Box::new(SyntaxShape::Number)),
                "The second vector to use in determining the dot product.",
            )
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Returns the dot product of two lists of numbers, interpreting both as vectors."
    }

    fn extra_description(&self) -> &str {
        "This is equivalent to a pairwise multiplication of both lists, followed by a summation."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "dot product"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Get the dot product of two vectors",
            example: "[1 2 3] | vec dot [3 4 -5]",
            result: Some(Value::test_int(-4)),
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
        compute_dot_product(
            vector_lhs,
            vector_rhs.as_slice(),
            pipeline_span,
            command_span,
        )
    })
}

pub fn compute_dot_product(
    vector_lhs: &[Value],
    vector_rhs: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    if vector_lhs.len() != vector_rhs.len() {
        return Err(
            LabeledError::new("Only equal-length vectors are supported.")
                .with_label(
                    format!("The pipeline contained {} elements.", vector_lhs.len()),
                    command_span,
                )
                .with_label(
                    format!("The list contained {} elements.", vector_rhs.len()),
                    command_span,
                ),
        );
    }

    let vector_element_pairs = vector_lhs.iter().zip(vector_rhs);
    let element_products: Vec<Value> = vector_element_pairs
        .map(|(pipeline_value, arg_value)| {
            pipeline_value
                .mul(command_span, arg_value, command_span)
                .unwrap_or(Value::float(0f64, command_span))
        })
        .collect_vec();

    sum(element_products, pipeline_span, command_span)
}

#[cfg(test)]
#[test]
fn test_examples() -> Result<(), ShellError> {
    PluginTest::new("nu_plugin_vec", VecPlugin.into())?.test_command_examples(&Command)
}
