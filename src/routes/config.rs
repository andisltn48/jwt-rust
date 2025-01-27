use actix_web::web;

use super::game::{get_games, create_game, delete_game, get_game_by_id, update_game};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_games)
        .service(create_game)
        .service(delete_game)
        .service(get_game_by_id)
        .service(update_game);

    conf.service(scope);
}