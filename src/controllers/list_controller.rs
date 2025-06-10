use crate::pre_import::*;

pub async fn list_users(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, Response> {
    let users = sqlx::query_as!(
        User,
        //"SELECT id, name, email FROM users"
        "SELECT * FROM users"  // Utilisation de SELECT * pour récupérer tous les champs
    )
    .fetch_all(&pool)
    .await;

    match users {
        Ok(data) => Ok(AxumJson(data)),
        Err(e) => {
            eprintln!("❌ Erreur lors de la récupération : {e}");
            let err_response = AxumJson(format!("Erreur interne : {e}"));
            Err((StatusCode::INTERNAL_SERVER_ERROR, err_response).into_response())
        }
    }

}


