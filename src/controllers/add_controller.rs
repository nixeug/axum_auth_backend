//
/// Handler pour ajouter un nouvel utilisateur
use crate::pre_import::*;
pub async fn add_user(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<impl IntoResponse, Response> {
    println!("📥 Données reçues : {:?}", payload);

    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        payload.name,
        payload.email
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, AxumJson("Utilisateur créé avec succès"))),
        Err(e) => {
            eprintln!("❌ Erreur lors de l'insertion : {e}");
            let err_response = AxumJson(format!("Erreur interne : {e}"));
            Err((StatusCode::INTERNAL_SERVER_ERROR, err_response).into_response())
        }
    }
}
