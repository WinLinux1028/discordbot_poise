use crate::{data::sns_post::Token, http_server::Server, Error};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier, TokenResponse};

pub async fn run(State(state): State<Server>, Query(query): Query<OAuthQuery>) -> Response {
    match run_(state, query).await {
        Ok(o) => o,
        Err(_) => (StatusCode::BAD_REQUEST, "400 Bad Request").into_response(),
    }
}

async fn run_(state: Server, query: OAuthQuery) -> Result<Response, Error> {
    let oauth_state: OAuthState = sqlx::query_as(
        "SELECT guildid, channelid, service, domain, code_verifier, client_id, client_secret FROM oauth2_state WHERE state=$1;"
    )
        .bind(&query.state)
        .fetch_optional(&state.psql)
        .await?
        .ok_or("")?;

    let client = if oauth_state.service == "Twitter" {
        state.twitter_client.ok_or("")?
    } else if oauth_state.service == "Mastodon" {
        crate::data::sns_post::mastodon::get_client(
            &state.hostname,
            &oauth_state.domain,
            oauth_state.client_id.ok_or("")?,
            oauth_state.client_secret.ok_or("")?,
        )?
    } else {
        return Err("".into());
    };

    let token = client
        .exchange_code(AuthorizationCode::new(query.code))
        .set_pkce_verifier(PkceCodeVerifier::new(oauth_state.code_verifier))
        .request_async(async_http_client)
        .await?;

    let token = Token::new(
        token.refresh_token(),
        token.access_token(),
        token.expires_in(),
    )?;

    let mut trx = state.psql.begin().await?;
    token
        .db_insert(
            &mut trx,
            &oauth_state.guildid,
            &oauth_state.channelid,
            &oauth_state.domain,
            &oauth_state.service,
        )
        .await?;
    sqlx::query("DELETE FROM oauth2_state WHERE state=$1;")
        .bind(&query.state)
        .execute(&mut *trx)
        .await?;
    trx.commit().await?;

    Ok("ログイン成功".into_response())
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthQuery {
    code: String,
    state: String,
}

#[derive(sqlx::FromRow)]
struct OAuthState {
    guildid: String,
    channelid: String,
    service: String,
    domain: String,
    code_verifier: String,
    client_id: Option<String>,
    client_secret: Option<String>,
}
