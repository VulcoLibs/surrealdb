#![allow(warnings)]

#[macro_use]
extern crate tracing;

#[macro_use]
mod mac;

mod cnf;
mod dbs;
mod env;
mod err;
mod gql;
mod rpc;
mod telemetry;

pub mod cli;
pub mod net;

pub use surrealdb_core;
pub use surrealdb;
