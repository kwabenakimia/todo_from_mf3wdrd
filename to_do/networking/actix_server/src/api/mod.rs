//! File: to_do/networking/actix_server/src/api/mod.rs
pub mod basic_actions;

use actix_web::web::ServiceConfig;
pub fn views_factory(app: &mut ServiceConfig) {
    basic_actions::basic_actions_factory(app);
}