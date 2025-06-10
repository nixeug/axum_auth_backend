use reqwest::StatusCode;

#[tokio::test]
async fn test_get_users_returns_200_and_valid_body() {
    let url = "http://localhost:3000/users";

    let response = reqwest::get(url)
        .await
        .expect("La requête a échoué");

    // Vérifie le code HTTP
    assert_eq!(response.status(), StatusCode::OK, "Status non OK");

    // Vérifie un header (exemple : Content-Type)
    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(content_type.contains("text/plain"), "Content-Type inattendu");

    // Vérifie le corps de la réponse
    let body = response.text().await.expect("Échec lecture du corps");
    assert!(
        body.contains("Users"),
        "Réponse inattendue : contenu ne contient pas 'Users'"
    );

    println!("Succès : {}", body);
}
