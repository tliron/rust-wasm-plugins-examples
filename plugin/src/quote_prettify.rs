use super::{
    bindings::{export, exports::acme::plugins::prettify_plugin},
    host,
};

// This is our implementation of the "prettify" plugin type (the WIT world)
//
// We shall make the content pretty by quoting all words!
//
// Gorgeous!!!!

//
// QuotePrettifyPlugin
//

pub struct QuotePrettifyPlugin;

export!(QuotePrettifyPlugin);

impl prettify_plugin::Guest for QuotePrettifyPlugin {
    // We must provide implementations for resources (see below)
    type PersonResource = Person;

    fn prettify(words: Vec<String>) -> Result<String, String> {
        host::log("thank you for calling prettify!");

        if words.is_empty() {
            return Err("I have no words to express my disappointment".into());
        }

        let quoted_words: Vec<String> = words.into_iter().map(|word| format!("{:?}", word)).collect();

        Ok(quoted_words.join(" "))
    }

    fn greet(person: prettify_plugin::PersonResource) -> Result<String, String> {
        host::log_structured("thank you for calling greet!", &[("plugin", "prettify"), ("implementation", "quote")]);

        // Convert to our implementation type
        let person: Person = person.into_inner();

        if person.first_name.is_empty() || person.last_name.is_empty() {
            return Err("person is incomplete".into());
        }

        let greeting = format!("Hello, {:?} {:?}!", person.first_name, person.last_name);

        Ok(greeting)
    }
}

//
// Person
//

pub struct Person {
    first_name: String,
    last_name: String,
}

impl prettify_plugin::GuestPersonResource for Person {
    fn new(first_name: String, last_name: String) -> Self {
        Self { first_name, last_name }
    }

    fn inner(&self) -> (String, String) {
        (self.first_name.clone(), self.last_name.clone())
    }
}
