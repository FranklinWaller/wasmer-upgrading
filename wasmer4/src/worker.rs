use actix::{prelude::*, Handler, Message};

use crate::{errors::Result, vm_manual};

pub struct RuntimeWorker;

impl Actor for RuntimeWorker {
    type Context = SyncContext<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct StartRuntimeWorker;

impl Handler<StartRuntimeWorker> for RuntimeWorker {
    type Result = Result<()>;

    fn handle(&mut self, _msg: StartRuntimeWorker, _ctx: &mut Self::Context) -> Self::Result {
        vm_manual::run_vm().unwrap();
        Ok(())
    }
}
