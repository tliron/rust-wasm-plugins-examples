use super::bindings::acme::plugins::host;

// Here we'll put convenience wrappers to make calling functions ergonomic
// by converting types to and from what the host expects

pub fn log(message: &str) {
    // (This one is trivial)
    host::log(message);
}

pub fn log_structured(message: &str, properties: &[(&str, &str)]) {
    let properties: Vec<(String, String)> =
        properties.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect();
    host::log_structured(message, host::MapResource::new(&properties));
}
