use nu_protocol::{LabeledError, ShellError, Span, Value};

pub fn sum(data: Vec<Value>, span: Span, head: Span) -> Result<Value, LabeledError> {
    let initial_value = data.first();

    let mut acc = match initial_value {
        Some(v) => {
            let span = v.span();
            match v {
                Value::Int { .. } | Value::Float { .. } => Ok(Value::int(0, span)),
                _ => Ok(Value::nothing(head)),
            }
        }

        None => Err(ShellError::UnsupportedInput {
            msg: "Empty input".to_string(),
            input: "value originates from here".into(),
            msg_span: head,
            input_span: span,
        }),
    }?;

    for value in &data {
        match value {
            Value::Int { .. }
            | Value::Float { .. }
            | Value::Filesize { .. }
            | Value::Duration { .. } => {
                acc = acc.add(head, value, head)?;
            }
            Value::Error { error, .. } => return Err(LabeledError::from(*error.clone())),
            other => {
                return Err(LabeledError::from(ShellError::UnsupportedInput {
                    msg: "Attempted to compute the sum of a value that cannot be summed"
                        .to_string(),
                    input: "value originates from here".into(),
                    msg_span: head,
                    input_span: other.span(),
                }));
            }
        }
    }
    Ok(acc)
}
