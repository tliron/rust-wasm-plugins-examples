#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../assets/wit/acme-plugins.wit",
    with: {
        "acme:plugins/host/map-resource": super::host::Map,
    },
    imports: { default: trappable },
});
