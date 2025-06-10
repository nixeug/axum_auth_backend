use crate::pre_import::*;
pub async fn get_user(
    State(pool): State<PgPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, Response> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(data) => Ok(axum::Json(data)),
        Err(e) => {
            eprintln!("❌ Erreur lors de la récupération de l'utilisateur : {e}");
            let err_response = AxumJson(format!("Erreur interne : {e}"));
            Err((StatusCode::INTERNAL_SERVER_ERROR, err_response).into_response())
        }
    }
}

