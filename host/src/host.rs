use super::bindings::acme::plugins::host;

use wasmtime_wasi::*;

//
// Host
//

/// Plugins host.
pub struct Host {
    wasi: WasiCtx,
    resources: ResourceTable,
}

impl Host {
    /// Constructor.
    pub fn new() -> Self {
        let wasi = WasiCtxBuilder::new().inherit_stdout().build();
        Self { wasi, resources: ResourceTable::new() }
    }
}

// We need to implement WasiView for wasmtime_wasi::add_to_linker_sync
impl WasiView for Host {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi, table: &mut self.resources }
    }
}

// Our exposed Host functions
impl host::Host for Host {
    // Note: the wasmtime::Result is because we added "imports: { default: trappable }" in bindgen
    fn log(&mut self, message: String) -> wasmtime::Result<()> {
        println!("log: {}", message);
        Ok(())
    }
}
