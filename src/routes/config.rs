use actix_web::web;

use super::game::{get_games, create_game, delete_game, get_game_by_id, update_game};
use super::auth::{login};

pub fn config(conf: &mut web::ServiceConfig) {
    let game_scope = web::scope("/api")
        .service(get_games)
        .service(create_game)
        .service(delete_game)
        .service(get_game_by_id)
        .service(update_game);

    let auth_scope = web::scope("/api")
        .service(login);

    conf.service(game_scope);
    conf.service(auth_scope);
}