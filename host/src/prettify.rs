use super::{bindings, host::*};

use {
    anyhow::*,
    std::path::*,
    wasmtime::component::{Component, HasSelf, Linker},
};

//
// Prettify
//

/// Prettify plugin.
pub struct Prettify {
    host: wasmtime::Store<Host>,
    prettify: bindings::Prettify,
}

// Wasmtime uses Anyhow for most of its errors
// But you could potentially wrap it in your own "PluginError" or similar using .map_err
// For this example we used Anyhow's .context to provide more information

impl Prettify {
    /// Constructor.
    pub fn new<PathT>(module: PathT) -> wasmtime::Result<Self>
    where
        PathT: AsRef<Path>,
    {
        let engine = wasmtime::Engine::default();

        // Host
        let mut host = wasmtime::Store::new(&engine, Host::new());

        // Component
        let component = Component::from_file(&engine, module).context("loading component from file")?;

        // Linker
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).context("adding WASI bindings")?;
        bindings::Prettify::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut Host| state)
            .context("adding prettify bindings")?;

        // Bindings
        let prettify = bindings::Prettify::instantiate(&mut host, &component, &linker)
            .context("instantiating prettify bindings")?;

        Ok(Self { host, prettify })
    }

    // Here we'll put convenience wrappers to make calling functions ergonomic
    // by converting types to and from what the plugin expects

    // Note: the outer wasmtime::Result is there because we added "imports: { default: trappable }" in bindgen
    // The inner Result is the one from our plugin

    /// Prettify words.
    pub fn prettify(&mut self, words: &[&str]) -> wasmtime::Result<Result<String, String>> {
        let words: Vec<String> = words.iter().map(|word| word.to_string()).collect();
        self.prettify.acme_plugins_prettify_plugin().call_prettify(&mut self.host, &words).context("calling prettify")
    }

    /// Greet a person.
    pub fn greet(&mut self, first_name: &str, last_name: &str) -> wasmtime::Result<Result<String, String>> {
        // Construct a guest's person resource
        let person_resource = self
            .prettify
            .acme_plugins_prettify_plugin()
            .person_resource()
            .call_constructor(&mut self.host, first_name, last_name)
            .context("calling person-resource constructor")?;

        self.prettify
            .acme_plugins_prettify_plugin()
            .call_greet(&mut self.host, person_resource)
            .context("calling greet")
    }
}
