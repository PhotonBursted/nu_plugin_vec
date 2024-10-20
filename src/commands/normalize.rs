use crate::commands::magnitude::compute_magnitude;
use crate::utils::process_pipeline;
use crate::VecPlugin;
use itertools::Itertools;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, Example, LabeledError, PipelineData, Signature, Span, Type, Value};

#[derive(Clone)]
pub struct Command;

impl PluginCommand for Command {
    type Plugin = VecPlugin;

    fn name(&self) -> &str {
        "vec normalize"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec normalize")
            .input_output_types(vec![(
                Type::List(Box::new(Type::Number)),
                Type::List(Box::new(Type::Number)),
            )])
            .allow_variants_without_examples(true)
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Scales an input vector, such that its magnitude equals 1."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "unit", "length"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Normalize a vector",
            example: "[2 3 6] | vec normalize",
            result: Some(Value::test_list(vec![
                Value::test_float(2.0 / 7.0),
                Value::test_float(3.0 / 7.0),
                Value::test_float(6.0 / 7.0),
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
        operate(call, input)
    }
}

fn operate(call: &EvaluatedCall, input: PipelineData) -> Result<PipelineData, LabeledError> {
    process_pipeline(call, input, |vector_lhs, pipeline_span, command_span| {
        normalize_vector(vector_lhs, pipeline_span, command_span)
    })
}

pub fn normalize_vector(
    vector: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    let magnitude = compute_magnitude(vector, pipeline_span, command_span)?;
    let output = Value::list(
        vector
            .iter()
            .map(|component| {
                component
                    .div(command_span, &magnitude, command_span)
                    .unwrap_or(Value::float(0f64, command_span))
            })
            .collect_vec(),
        command_span,
    );

    Ok(output)
}
