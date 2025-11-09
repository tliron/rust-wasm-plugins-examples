#![allow(missing_docs)]

wasmtime::component::bindgen!({
    path: "../assets/wit/acme-plugins.wit",
    // with: {
    //     "acme:plugins/prettify-plugin/list-resource": super::list::List,
    // },
    imports: { default: trappable },
});
