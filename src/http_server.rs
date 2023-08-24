mod oauth;

use axum::{routing, Router};
use oauth2::basic::BasicClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Server {
    psql: PgPool,
    twitter_client: Option<BasicClient>,
}

pub async fn start(listen: String, psql: PgPool, twitter_client: Option<BasicClient>) {
    let state = Server {
        psql,
        twitter_client,
    };

    let app = Router::new()
        .route("/oauth", routing::get(oauth::run))
        .with_state(state);

    axum::Server::bind(&listen.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
