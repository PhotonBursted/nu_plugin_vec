use nu_protocol::{LabeledError, ShellError, Span, Value};

pub fn assert_equal_length_vectors(vector_lhs: &[Value], vector_rhs: &[Value], pipeline_span: Span, command_span: Span) -> Option<LabeledError> {
    if vector_lhs.len() != vector_rhs.len() {
        return Some(LabeledError::from(ShellError::IncorrectValue {
            msg: format!("Only equal-length vectors are supported.\nThe pipeline contained {} elements, the list contained {}.", vector_lhs.len(), vector_rhs.len()),
            val_span: pipeline_span,
            call_span: command_span,
        }));
    }

    None
}