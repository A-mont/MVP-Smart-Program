
#![no_std]

use sails_rs::prelude::*;

pub mod services;

use services::service::Service;

pub struct Program;

#[program]
impl Program {
    pub fn new(admin: ActorId) -> Self {
        Service::seed(admin);
        Self
    }

    #[route("Service")]
    pub fn service_svc(&self) -> Service {
        Service::new()
    }
}
