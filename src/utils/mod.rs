pub(crate) mod reducers;

use nu_plugin::EvaluatedCall;
use nu_protocol::{IntoPipelineData, LabeledError, PipelineData, ShellError, Span, Value};

pub fn process_pipeline(
    call: &EvaluatedCall,
    input: PipelineData,
    mf: impl Fn(&[Value], Span, Span) -> Result<Value, LabeledError>,
) -> Result<PipelineData, LabeledError> {
    let name = call.head;
    let res = calculate(input, name, mf);
    match res {
        Ok(v) => Ok(v.into_pipeline_data()),
        Err(e) => Err(LabeledError::from(e)),
    }
}

pub fn calculate(
    values: PipelineData,
    name: Span,
    mf: impl Fn(&[Value], Span, Span) -> Result<Value, LabeledError>,
) -> Result<Value, LabeledError> {
    let span = values.span().unwrap_or(name);
    match values {
        PipelineData::Value(Value::List { ref vals, .. }, ..) => match &vals[..] {
            _ => mf(vals, span, name),
        },
        PipelineData::Empty { .. } => Err(LabeledError::from(ShellError::PipelineEmpty {
            dst_span: name,
        })),
        val => Err(LabeledError::from(ShellError::UnsupportedInput {
            msg: "Only lists of numbers are supported".into(),
            input: "value originates from here".into(),
            msg_span: name,
            input_span: val
                .span()
                .expect("non-Empty non-ListStream PipelineData had no span"),
        })),
    }
}
