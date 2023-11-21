use std::time::Duration;

use actix::prelude::*;

use crate::errors::Result;
use crate::worker::{RuntimeWorker, StartRuntimeWorker};

pub struct App {
    pub worker: Addr<RuntimeWorker>,
}

impl App {
    pub fn new() -> Self {
        let worker = SyncArbiter::start(10, || RuntimeWorker);

        Self { worker }
    }
}

impl Actor for App {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started Actix");

        ctx.address().do_send(StartVm);
    }
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
struct StartVm;

impl Handler<StartVm> for App {
    type Result = Result<()>;

    fn handle(&mut self, _msg: StartVm, ctx: &mut Self::Context) -> Self::Result {
        self.worker.do_send(StartRuntimeWorker);
        ctx.notify_later(StartVm, Duration::from_secs(5));
        Ok(())
    }
}
