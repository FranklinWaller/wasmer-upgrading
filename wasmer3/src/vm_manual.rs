use wasmer::{FunctionEnv, Instance, Store};
use wasmer_wasix::WasiEnv;

use crate::{
    host_adapter::VmAdapter,
    imports::{create_imports, VmContext},
    wasm_cache::{wasm_cache_id, wasm_cache_load, wasm_cache_store},
};

pub fn run_vm() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../cowsay.wasm");
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = std::fs::read(wasm_path)?;

    // Create a Store.
    let mut store = Store::default();

    println!("Compiling module...");
    let wasm_id = wasm_cache_id(&wasm_bytes);

    // Let's compile the Wasm module.
    let module = match wasm_cache_load(&store, &wasm_id) {
        Ok(module) => module,
        Err(_) => wasm_cache_store(&store, &wasm_id, wasm_bytes).unwrap(),
    };

    println!("Creating `WasiEnv`...");
    // First, we create the `WasiEnv`
    let mut wasi_env = WasiEnv::builder("cowsay")
        .args(["world"])
        // .env("KEY", "Value")
        .finalize(&mut store)?;

    println!("Instantiating module with WASI imports...");
    // Then, we get the import object related to our WASI
    // and attach it to the Wasm instance.
    let vm_context = FunctionEnv::new(&mut store, VmContext::new(wasi_env.env.clone()));
    let mut import_object = wasi_env.import_object(&mut store, &module)?;
    let host_adapter = VmAdapter {};

    let custom_imports = create_imports(&mut store, &vm_context, host_adapter);
    import_object.register_namespace("env", custom_imports.get_namespace_exports("env").unwrap());

    let instance = Instance::new(&mut store, &module, &import_object)?;

    println!("Attach WASI memory...");

    let env_mut = vm_context.as_mut(&mut store);
    let memory = instance.exports.get_memory("memory")?;
    env_mut.memory = Some(memory.clone());

    wasi_env.initialize(&mut store, instance.clone())?;

    println!("Call WASI `_start` function...");
    // And we just call the `_start` function!
    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    println!("Done");

    wasi_env.cleanup(&mut store, None);

    Ok(())
}
