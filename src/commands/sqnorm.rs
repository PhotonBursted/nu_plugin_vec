use crate::commands::dot::compute_dot_product;
use crate::utils::process_pipeline;
use crate::VecPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Span, Type, Value};

#[derive(Clone)]
pub struct Command;

impl PluginCommand for Command {
    type Plugin = VecPlugin;

    fn name(&self) -> &str {
        "vec sqnorm"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec sqnorm")
            .input_output_types(vec![(Type::List(Box::new(Type::Number)), Type::Number)])
            .allow_variants_without_examples(true)
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Returns the squared norm of a list of numbers, interpreting it as if it were a vector."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "squared norm"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Calculate the squared norm of a vector",
            example: "[1 2 3] | vec sqnorm",
            result: Some(Value::test_int(14)),
        }]
    }

    fn run(
        &self,
        _plugin: &VecPlugin,
        _engine_state: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        operate(call, input)
    }
}

fn operate(call: &EvaluatedCall, input: PipelineData) -> Result<PipelineData, LabeledError> {
    process_pipeline(call, input, |vector_lhs, pipeline_span, command_span| {
        compute_squared_norm(vector_lhs, pipeline_span, command_span)
    })
}

pub fn compute_squared_norm(
    vector: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    compute_dot_product(vector, vector, pipeline_span, command_span)
}
