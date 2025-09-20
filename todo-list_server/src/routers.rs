use crate::handlers::{general::*, todo_list::*};
use actix_web::web;

pub fn general_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn todo_list_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todos").route("/", web::get().to(get_all_todo_list)));
}
