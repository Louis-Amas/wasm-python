use std::path::Path;

use anyhow::{anyhow, Result};
use wasmtime::component::{bindgen, HasSelf};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::p2::add_to_linker_sync;
use wasmtime_wasi::{
    DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView,
};

bindgen!({ world: "strategy", path: "../wasm-python/strategy.wit" });

#[derive(Default)]
pub struct WasmStateWrapper {
    pub state: Vec<u8>,
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for WasmStateWrapper {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl StrategyImports for WasmStateWrapper {
    fn indicator(&mut self, _key: String) -> Result<Vec<u8>, String> {
        Ok(Default::default())
    }
}

pub struct Runtime {
    engine: Engine,
    store: Store<WasmStateWrapper>,
    linker: Linker<WasmStateWrapper>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(false);
        config.wasm_component_model(true);
        config.debug_info(false);
        let engine = Engine::new(&config)?;

        let mut builder = WasiCtxBuilder::new();
        builder
            .inherit_stdio()
            .preopened_dir(
                Path::new(
                    "/home/arconec/Tests/wasm-anera/bin/runner/target/wasm32-wasi/wasi-deps/usr",
                ),
                "/usr",
                DirPerms::all(),
                FilePerms::all(),
            )
            .unwrap();

        let wasi_ctx = builder.build();
        let resource_table = ResourceTable::new();

        let store = Store::new(
            &engine,
            WasmStateWrapper {
                state: Vec::new(),
                wasi_ctx,
                resource_table,
            },
        );

        let mut linker: Linker<WasmStateWrapper> = Linker::new(&engine);
        add_to_linker_sync(&mut linker)?;
        Strategy::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut _| state)?;

        Ok(Self {
            engine,
            store,
            linker,
        })
    }

    pub fn instantiate_strategy(&mut self, wasm: &[u8]) -> Result<Strategy> {
        let component = unsafe { Component::deserialize(&self.engine, wasm)? };
        Strategy::instantiate(&mut self.store, &component, &self.linker)
    }

    pub fn execute(&mut self, instance: &Strategy) -> Result<()> {
        const STRATEGY_CODE: &str =
            include_str!("/home/arconec/Tests/wasm-anera/bin/runner/src/py-func.py");

        // call_exec returns Result<Result<_, String>, anyhow::Error>
        instance
            .call_exec(&mut self.store, STRATEGY_CODE)
            .map_err(|e| anyhow!(e))?
            .map_err(|guest_err| anyhow!(guest_err))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Runtime;

    const STRATEGY_BYTES: &[u8] = include_bytes!("/home/arconec/Tests/wasm-anera/runner.cwasm");

    #[test]
    fn wasm_strategy() {
        let mut runtime = Runtime::new().unwrap();

        println!("instantiate");
        let strategy = runtime.instantiate_strategy(STRATEGY_BYTES).unwrap();
        println!("execute");
        runtime.execute(&strategy).unwrap();
    }
}
