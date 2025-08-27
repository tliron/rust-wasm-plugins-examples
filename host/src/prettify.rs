use super::{bindings, host::*};

use {
    anyhow::*,
    std::path::*,
    wasmtime::{
        component::{Component, Linker, *},
        *,
    },
};

//
// Prettify
//

/// Prettify plugin.
pub struct Prettify {
    store: Store<Host>,
    prettify: bindings::Prettify,
}

// Wasmtime uses Anyhow for most of its errors
// But you could potentially wrap it in your own "PluginError" or similar using .map_err
// For this example we used .context

impl Prettify {
    /// Constructor.
    pub fn new<PathT>(module: PathT) -> Result<Self, Error>
    where
        PathT: AsRef<Path>,
    {
        let engine = Engine::default();

        // Component
        let component = Component::from_file(&engine, module).context("load component")?;

        // Linker
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).context("link WASI")?;
        bindings::Prettify::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut Host| state)
            .context("link plugin host")?;

        // Store
        let mut store = Store::new(&engine, Host::new());

        // Bindings
        let prettify =
            bindings::Prettify::instantiate(&mut store, &component, &linker).context("instantiate bindings")?;

        Ok(Self { store, prettify })
    }

    // We'll create convenience wrappers to make calling functions ergonomic:

    /// Prettify.
    pub fn prettify(&mut self, name: &str) -> Result<String, Error> {
        self.prettify.acme_plugins_prettify_plugin().call_prettify(&mut self.store, name)
    }
}
