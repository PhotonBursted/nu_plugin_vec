use crate::commands::sqnorm::compute_squared_norm;
use crate::utils::process_pipeline;
use crate::VecPlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
#[cfg(test)]
use nu_plugin_test_support::PluginTest;
#[cfg(test)]
use nu_protocol::ShellError;
use nu_protocol::{
    Category, Example, IntoValue, LabeledError, PipelineData, Signature, Span, Type, Value,
};

#[derive(Clone)]
pub struct Command;

impl PluginCommand for Command {
    type Plugin = VecPlugin;

    fn name(&self) -> &str {
        "vec magnitude"
    }

    fn signature(&self) -> Signature {
        Signature::build("vec magnitude")
            .input_output_types(vec![(Type::List(Box::new(Type::Number)), Type::Number)])
            .allow_variants_without_examples(true)
            .category(Category::Math)
    }

    fn description(&self) -> &str {
        "Returns the magnitude of a vector, with elements as present in the pipeline."
    }

    fn extra_description(&self) -> &str {
        "This is equivalent to `(vec sqnorm | math sqrt)`."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["vector", "length"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Calculate the magnitude of a vector",
            example: "[2 3 6] | vec magnitude",
            result: Some(Value::test_int(7)),
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
        compute_magnitude(vector_lhs, pipeline_span, command_span)
    })
}

pub fn compute_magnitude(
    vector: &[Value],
    pipeline_span: Span,
    command_span: Span,
) -> Result<Value, LabeledError> {
    let squared_norm = compute_squared_norm(vector, pipeline_span, command_span)?;
    let output = squared_norm
        .coerce_float()
        .map(|float| float.sqrt().into_value(command_span));

    output.map_err(LabeledError::from)
}

#[cfg(test)]
#[test]
fn test_examples() -> Result<(), ShellError> {
    PluginTest::new("nu_plugin_vec", VecPlugin.into())?.test_command_examples(&Command)
}
