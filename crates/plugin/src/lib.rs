wit_bindgen::generate!(in "../../assets/wit/acme-plugins.wit");

use {acme::plugins::host, exports::acme::plugins::prettify_plugin};

// This is our implementation of the "prettify" plugin type (=WIT world)

// We shall make the content pretty by quoting all words
// Gorgeous!

pub struct QuotePrettifyPlugin;

impl prettify_plugin::Guest for QuotePrettifyPlugin {
    // Note: these are not "std::string:String", but "alloc::String"!
    fn prettify(content: String) -> String {
        host::log("thank you for using the quote prettify plugin!");
        let words = content.split(" ");
        let words: Vec<String> = words.map(|word| format!("{:?}", word)).collect();
        words.join(" ")
    }
}

export!(QuotePrettifyPlugin);
