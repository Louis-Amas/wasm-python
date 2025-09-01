use std::path::Path;

use anyhow::{anyhow, Result};
use wasmtime::component::bindgen;
use wasmtime::{
    component::{Component, Linker, ResourceTable},
    Config, Engine, Store,
};
use wasmtime_wasi::p2::{add_to_linker_sync, IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi::{DirPerms, FilePerms};

bindgen!({world: "strategy", path: "../wasm-python/strategy.wit", async: false,});

#[derive(Debug, Default)]
pub struct WasmStateWrapper {
    pub state: Vec<u8>,
}

impl StrategyImports for WasmStateWrapper {
    fn indicator(&mut self, key: String) -> Result<Vec<u8>, String> {
        return Ok(Default::default());
    }
}

struct WasmState {
    ctx: WasiCtx,
    table: ResourceTable,
    state: WasmStateWrapper,
}

impl IoView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for WasmState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

pub struct Runtime {
    engine: Engine,
    linker: Linker<WasmState>,
    store: Store<WasmState>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.async_support(false);
        config.wasm_component_model(true);
        config.debug_info(true);
        let engine = Engine::new(&config)?;

        let mut builder = WasiCtxBuilder::new();

        // Mount host path into guest /usr
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

        let store = Store::new(
            &engine,
            WasmState {
                ctx: builder.build(),
                table: ResourceTable::new(),
                state: Default::default(),
            },
        );

        let mut linker: Linker<WasmState> = Linker::new(&engine);

        add_to_linker_sync(&mut linker)?;
        Strategy::add_to_linker(&mut linker, |ctx| &mut ctx.state)?;

        Ok(Self {
            engine,
            store,
            linker,
        })
    }
    pub fn instantiate_strategy(&mut self, wasm: &[u8]) -> Result<Strategy> {
        println!("instantiate");
        let component = Component::from_binary(&self.engine, wasm)?;
        Strategy::instantiate(&mut self.store, &component, &self.linker)
    }

    pub fn execute(&mut self, instance: &Strategy) -> Result<()> {
        const STRATEGY_CODE: &str =
            include_str!("/home/arconec/Tests/wasm-anera/bin/runner/src/py-func.py");
        let results = instance.call_exec(&mut self.store, STRATEGY_CODE);
        println!("{:?}", results);

        results
            .map_err(|err| anyhow!(err))?
            .map_err(|err| anyhow!(err))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Runtime;

    const STRATEGY_BYTES: &[u8] =
        include_bytes!("/home/arconec/Tests/wasm-anera/target/wasm32-wasip1/debug/runner.wasm");

    #[test]
    fn wasm_strategy() {
        let mut runtime = Runtime::new().unwrap();

        let strategy = runtime.instantiate_strategy(STRATEGY_BYTES).unwrap();
        runtime.execute(&strategy).unwrap();
    }
}
