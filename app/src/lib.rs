
#![no_std]

use sails_rs::prelude::*;

pub mod services;

use services::service::Service;

pub struct MVPMProgram;

#[program]
impl MVPMProgram {
    pub fn new() -> Self {
        Service::seed();
        Self
    }

    #[route("Service")]
    pub fn service_svc(&self) -> Service {
        Service::new()
    }
}
