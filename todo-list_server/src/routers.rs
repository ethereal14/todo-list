use crate::handlers::{general::*, todo_item::*, todo_list::*};
use actix_web::web;

pub fn general_router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn todo_list_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("/", web::get().to(get_all_todo_list))
            .route("/", web::post().to(post_new_todo_list))
            .route("/{id}", web::get().to(get_todo_list_detail))
            .route("/{id}", web::delete().to(delete_todo_list_by_id))
            .route("/{id}", web::put().to(update_todo_list_by_id))
            .route("/{id}", web::post().to(post_new_todo_item))
            .route(
                "/{list_id}/{item_id}",
                web::delete().to(delete_todo_item_by_id),
            )
            .route(
                "/{list_id}/{item_id}",
                web::put().to(update_todo_item_by_id),
            ),
    );
}
