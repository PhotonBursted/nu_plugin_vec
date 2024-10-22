use nu_plugin_test_support::PluginTest;
use nu_plugin_vec::commands::Scale as Command;
use nu_plugin_vec::{nu, test_examples, VecPlugin};
use nu_protocol::ShellError;

test_examples!();

mod scale_vector_stretching {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn errors_when_factor_vector_is_shorter() {
        let result = nu!("\
                let vector =  [1 2 3 4 5];\
                let factors = [1 2 3];\
                \
                $vector | vec scale $factors");

        assert!(result.is_err());
    }

    #[test]
    fn errors_when_target_vector_is_shorter() {
        let result = nu!("\
                let vector =  [1 2];\
                let factors = [1 2 3 4 5];\
                \
                $vector | vec scale $factors");

        assert!(result.is_err());
    }

    #[test]
    fn keeps_ints_where_possible_when_using_int_factors() {
        let result = nu!("\
                let vector =  [1 2.5];\
                let factors = [1  2 ];\
                \
                $vector | vec scale $factors");
        assert!(result.is_ok(), "{}", result.err().unwrap());

        let pipeline_data = result.unwrap();
        let result_elements = pipeline_data.into_iter().collect_vec();
        let first_element = result_elements.first().unwrap();
        assert!(first_element.as_int().is_ok());
    }
}

mod scale_vector_uniform {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn keeps_ints_where_possible_when_using_int_factor() {
        let result = nu!("\
                let vector = [1 2.5];\
                let factor = 2;\
                \
                $vector | vec scale $factor");
        assert!(result.is_ok(), "{}", result.err().unwrap());

        let pipeline_data = result.unwrap();
        let result_elements = pipeline_data.into_iter().collect_vec();
        let first_element = result_elements.first().unwrap();
        assert!(first_element.as_int().is_ok());
    }
}
