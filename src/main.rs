#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use()]
extern crate rocket;

#[macro_use()]
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;
use rocket_cors::{CorsOptions, AllowedOrigins}
use dotenv::dotenv

mod auth;
mod controllers;
mod models;
mod schema;

use diesel::prelude::*;
use diesel::PgConnection;
use std:: env

#[rocker::main]
async fn main() {
    rocket::build()
        .attach()
}
