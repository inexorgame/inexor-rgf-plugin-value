use crate::model::NamespacedTypeGetter;
use crate::plugins::ComponentProvider;
use crate::providers::ValueComponentProviderImpl;

#[test]
fn components_should_exist() {
    let expected_components = vec![
        "state_array",
        "state_boolean",
        "state_debugger_debug",
        "state_debugger_trace",
        "state_number",
        "state_object",
        "state_string",
        "value_array",
        "value_boolean",
        "value_debugger_debug",
        "value_debugger_trace",
        "value_number",
        "value_object",
        "value_string",
    ];
    let component_provider = ValueComponentProviderImpl {};
    let components = component_provider.get_components();
    assert_eq!(expected_components.len(), components.len());
    assert_eq!(
        expected_components.len(),
        components
            .into_iter()
            .filter(|component| expected_components.contains(&component.type_name().as_str()))
            .count()
    );
}
