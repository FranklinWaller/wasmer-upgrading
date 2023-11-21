mod errors;
mod host_adapter;
mod imports;
mod vm_manual;
mod wasm_cache;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    vm_manual::run_vm().unwrap();

    // run_vm().unwrap();

    Ok(())
}
