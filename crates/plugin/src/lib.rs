wit_bindgen::generate!(in "../../assets/wit/acme-plugins.wit");

use {acme::plugins::host, exports::acme::plugins::prettify_plugin};

pub struct QuotePrettifyPlugin;

impl prettify_plugin::Guest for QuotePrettifyPlugin {
    // Note: these are not "std::string:String", but "alloc::String"!
    fn prettify(name: String) -> String {
        host::log("thank you for using the quote prettify plugin!");
        let words = name.split(" ");
        let words: Vec<String> = words.map(|word| format!("{:?}", word)).collect();
        words.join(" ")
    }
}

export!(QuotePrettifyPlugin);
