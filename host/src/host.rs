use super::bindings::acme::plugins::host;

use {
    std::{collections::*, mem::*},
    wasmtime_wasi::*,
};

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
// Note: the wasmtime::Result is because we added "imports: { default: trappable }" in bindgen

impl host::Host for Host {
    fn log(&mut self, message: String) -> wasmtime::Result<()> {
        println!(">>> {}", message);
        Ok(())
    }

    fn log_structured(
        &mut self,
        message: String,
        properties: wasmtime::component::Resource<host::MapResource>,
    ) -> wasmtime::Result<()> {
        let properties = &self.resources.get(&properties)?.inner;
        println!(">>> {}", message);
        for (key, value) in properties {
            println!("  {}: {}", key, value);
        }
        Ok(())
    }
}

// We must provide implementations for resources (see below)

impl host::HostMapResource for Host {
    fn new(
        &mut self,
        key_value_pairs: Vec<(String, String)>,
    ) -> wasmtime::Result<wasmtime::component::Resource<host::MapResource>> {
        Ok(self.resources.push(Map::new(key_value_pairs))?)
    }

    fn drop(&mut self, map_resource: wasmtime::component::Resource<host::MapResource>) -> wasmtime::Result<()> {
        self.resources.delete(map_resource)?;
        Ok(())
    }

    fn take(
        &mut self,
        map_resource: wasmtime::component::Resource<host::MapResource>,
    ) -> wasmtime::Result<Vec<(String, String)>> {
        Ok(take(&mut self.resources.get_mut(&map_resource)?.inner)
            .into_iter()
            .map(|(key, value)| (key, value))
            .collect())
    }

    fn length(&mut self, map_resource: wasmtime::component::Resource<host::MapResource>) -> wasmtime::Result<u64> {
        let map = self.resources.get(&map_resource)?;
        Ok(map.inner.len() as u64)
    }
}

//
// Map
//

/// Map.
pub struct Map {
    inner: BTreeMap<String, String>,
}

impl Map {
    pub fn new(key_value_pairs: Vec<(String, String)>) -> Self {
        Self { inner: key_value_pairs.into_iter().collect() }
    }
}
