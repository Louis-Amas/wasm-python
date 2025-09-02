#[cfg(feature = "wasm")]
fn prepare_cpyton_wasm() {
    use wlr_libpy::bld_cfg::configure_static_libs;
    configure_static_libs().unwrap().emit_link_flags();
}

fn main() {
    #[cfg(feature = "wasm")]
    prepare_cpyton_wasm();
}
