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
    store: wasmtime::Store<Host>,
    prettify: bindings::Prettify,
}

// Wasmtime uses Anyhow for most of its errors
// But you could potentially wrap it in your own "PluginError" or similar using .map_err
// For this example we used .context

impl Prettify {
    /// Constructor.
    pub fn new<PathT>(module: PathT) -> wasmtime::Result<Self>
    where
        PathT: AsRef<Path>,
    {
        let engine = wasmtime::Engine::default();

        // Component
        let component = Component::from_file(&engine, module).context("load component")?;

        // Linker
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).context("link WASI")?;
        bindings::Prettify::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut Host| state)
            .context("link plugin host")?;

        // Store
        let mut store = wasmtime::Store::new(&engine, Host::new());

        // Bindings
        let prettify =
            bindings::Prettify::instantiate(&mut store, &component, &linker).context("instantiate bindings")?;

        Ok(Self { store, prettify })
    }

    // Here we'll put convenience wrappers to make calling functions ergonomic:
    // Note: the wasmtime::Result is because we added "imports: { default: trappable }" in bindgen

    /// Prettify.
    pub fn prettify(&mut self, content: &str) -> wasmtime::Result<Result<String, String>> {
        self.prettify.acme_plugins_prettify_plugin().call_prettify(&mut self.store, content).context("call prettify")
    }

    /// Prettify words.
    pub fn prettify_words(&mut self, words: Vec<&str>) -> wasmtime::Result<Result<String, String>> {
        let words: Vec<String> = words.iter().map(|word| word.to_string()).collect();

        // Construct a guest's list resource
        let list_resource = self
            .prettify
            .acme_plugins_prettify_plugin()
            .list_resource()
            .call_constructor(&mut self.store, &words)
            .context("call list-resource constructor")?;

        self.prettify
            .acme_plugins_prettify_plugin()
            .call_prettify_words(&mut self.store, list_resource)
            .context("call prettify-words")
    }
}
