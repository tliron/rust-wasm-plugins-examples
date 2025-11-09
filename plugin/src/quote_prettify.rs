use super::bindings::{acme::plugins::host, export, exports::acme::plugins::prettify_plugin};

// This is our implementation of the "prettify" plugin type (the WIT world)
//
// We shall make the content pretty by quoting all words!
//
// Gorgeous!!!!

pub struct QuotePrettifyPlugin;

export!(QuotePrettifyPlugin);

impl prettify_plugin::Guest for QuotePrettifyPlugin {
    // We must provide implementations for resources
    type ListResource = List;

    fn prettify(content: String) -> Result<String, String> {
        host::log("thank you for calling prettify!");

        if content.is_empty() {
            return Err("content is empty".into());
        }

        let words = content.split(" ");
        let words: Vec<String> = words.map(|word| format!("{:?}", word)).collect();

        Ok(words.join(" "))
    }

    fn prettify_words(words: prettify_plugin::ListResource) -> Result<String, String> {
        host::log("thank you for calling prettify-words!");

        // Convert to our implementation type
        let content: List = words.into_inner();

        if content.inner.is_empty() {
            return Err("content is empty".into());
        }

        let words = &content.inner;
        let words: Vec<String> = words.iter().map(|word| format!("{:?}", word)).collect();

        Ok(words.join(" "))
    }
}

#[derive(Clone)]
pub struct List {
    inner: Vec<String>,
}

impl prettify_plugin::GuestListResource for List {
    fn new(list: Vec<String>) -> Self {
        Self { inner: list }
    }

    fn get(&self) -> Vec<String> {
        self.clone().inner
    }

    fn length(&self) -> u64 {
        self.inner.len() as u64
    }
}
