

#[get("/api/games")]
pub async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let games = sqlx::query_as!(
        GameModel,
        "SELECT * FROM games ORDER BY id ASC"
    )
    .fetch_all(&data.db())
    .await;
    
    if let Ok(games) = games {
        let games = games.unwrap();

        let response = serde_json::json!({"status": "success", "data": {"games": games}});
        return HttpResponse::Ok().json(response);
    }
    return HttpResponse::InternalServerError().json(json!("Internal server error: {}",));
}