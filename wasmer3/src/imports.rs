use parking_lot::RwLock;
use std::sync::Arc;
use wasmer_wasix::WasiEnv;

use crate::host_adapter::HostAdapter;
use wasmer::{
    imports, AsStoreRef, Function, FunctionEnv, FunctionEnvMut, Imports, Memory, MemoryView, Store,
    WasmPtr,
};

use crate::errors::Result;

pub struct VmContext {
    pub call_result_value: Arc<RwLock<Vec<u8>>>,
    pub memory: Option<Memory>,
    pub wasi_env: FunctionEnv<WasiEnv>,
}

impl VmContext {
    pub fn new(wasi_env: FunctionEnv<WasiEnv>) -> Self {
        Self {
            call_result_value: Default::default(),
            memory: None,
            wasi_env,
        }
    }

    pub fn memory_view<'a>(&'a self, store: &'a impl AsStoreRef) -> MemoryView<'a> {
        self.memory().view(store)
    }

    pub fn memory(&self) -> &Memory {
        self.memory.as_ref().unwrap()
    }
}

pub fn call_result_value_write_import_obj(
    store: &mut Store,
    vm_context: &FunctionEnv<VmContext>,
) -> Function {
    fn call_result_value(
        env: FunctionEnvMut<'_, VmContext>,
        result_data_ptr: WasmPtr<u8>,
        result_data_length: u32,
    ) -> Result<()> {
        let ctx = env.data();
        let memory = ctx.memory_view(&env);

        let target = result_data_ptr.slice(&memory, result_data_length)?;
        let call_value = ctx.call_result_value.read();

        for index in 0..result_data_length {
            target
                .index(index as u64)
                .write(call_value[index as usize])?;
        }

        Ok(())
    }

    Function::new_typed_with_env(store, vm_context, call_result_value)
}

pub fn http_fetch_import_obj(
    store: &mut Store,
    vm_context: &FunctionEnv<VmContext>,
    host_adapter: impl HostAdapter,
) -> Function {
    Function::new_typed_with_env(
        store,
        vm_context,
        move |env: FunctionEnvMut<'_, VmContext>,
              action_ptr: WasmPtr<u8>,
              action_length: u32|
              -> Result<u32> {
            let ctx = env.data();
            let memory = ctx.memory_view(&env);
            let wasi_env = ctx.wasi_env.as_ref(&env);
            let action_raw = action_ptr.slice(&memory, action_length)?.read_to_bytes()?;
            let url = String::from_utf8(action_raw.to_vec())?;
            let ha_clone = host_adapter.clone();

            let result: String = wasi_env
                .tasks()
                .block_on(async move { ha_clone.http_fetch(url).await.unwrap() });

            let mut call_value = ctx.call_result_value.write();
            *call_value = serde_json::to_vec(&result)?;

            Ok(call_value.len() as u32)
        },
    )
}

pub fn call_result_value_length_import_obj(
    store: &mut Store,
    vm_context: &FunctionEnv<VmContext>,
) -> Function {
    fn call_result_value_length(env: FunctionEnvMut<'_, VmContext>) -> Result<u32> {
        let ctx = env.data();
        let call_value = ctx.call_result_value.read();

        Ok(call_value.len() as u32)
    }

    Function::new_typed_with_env(store, vm_context, call_result_value_length)
}

pub fn create_imports(
    store: &mut Store,
    vm_context: &FunctionEnv<VmContext>,
    host_adapter: impl HostAdapter,
) -> Imports {
    imports! {
        "env" => {
            "call_result_write" => call_result_value_write_import_obj(store, vm_context),
            "call_result_length" => call_result_value_length_import_obj(store, vm_context),
            "http_fetch" => http_fetch_import_obj(store, vm_context, host_adapter),
        }
    }
}
