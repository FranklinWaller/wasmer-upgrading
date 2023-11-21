use actix::prelude::*;
use app::App;

mod app;
mod errors;
mod host_adapter;
mod imports;
mod vm_manual;
mod wasm_cache;
mod worker;

fn main() -> anyhow::Result<()> {
    let system = System::new();

    println!("Warm up run");
    vm_manual::run_vm().unwrap();

    system.block_on(async {
        let app = App::new();
        app.start();
    });

    let code = system.run_with_code();
    std::process::exit(code.expect("Actix should return an exit code"));

    // vm_manual::run_vm().unwrap();

    // run_vm().unwrap();
}
