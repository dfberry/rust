//! Example OAuth (GitHub) implementation.
//!
//! 1) Create a new application GitHub
//! 2) Visit the OAuth2 tab to get your CLIENT_ID and CLIENT_SECRET
//! 3) Add a new redirect URI (for this example: `http://127.0.0.1:3005/auth/authorized`)
//! 4) Run with the following (replacing values appropriately):
//! ```not_rust
//! CLIENT_ID=REPLACE_ME CLIENT_SECRET=REPLACE_ME cargo run -p example-oauth
//! ```
use std::env;
use urlencoding::encode;
use dotenv::dotenv;
use anyhow::{Context, Result};
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    response::Html,
    async_trait,
    extract::{FromRef, FromRequestParts, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    RequestPartsExt, Router,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use http::{header, request::Parts, StatusCode};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
<<<<<<< HEAD
use async_session::serde_json;
=======
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static COOKIE_NAME: &str = "SESSION";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_oauth=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    // `MemoryStore` is just used as an example. Don't use this in production.
    let store = MemoryStore::new();
    let oauth_client = oauth_client().unwrap();
    let app_state = AppState {
        store,
        oauth_client,
    };

    let app = Router::new()
<<<<<<< HEAD
        .route("/", get(index_handler))
        .route("/auth/github", get(login_authorized_handler))
        //.route("/auth/authorized", get(login_authorized_handler))
=======
        .route("/", get(login_handler))
        .route("/auth/github", get(github_auth_handler))
        .route("/auth/authorized", get(login_authorized_handler))
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
        .route("/protected", get(protected_handler))
        .route("/logout", get(logout_handler))
        .route("/login", get(login_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

        println!("listening on {}", listener.local_addr().unwrap());

    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .context("failed to return local address")
            .unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    store: MemoryStore,
    oauth_client: BasicClient,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

fn oauth_client() -> Result<BasicClient, AppError> {

    let client_id = env::var("GITHUB_CLIENT_ID").expect("oauth GITHUB_CLIENT_ID must be set");
    let client_secret = env::var("GITHUB_PASSWORD").expect("oauth GITHUB_PASSWORD must be set");
    let redirect_url = env::var("GITHUB_REDIR_URL").expect("oauth GITHUB_REDIR_URL must be set");
    let auth_url = env::var("GITHUB_AUTH_URL").expect("oauth GITHUB_AUTH_URL must be set");
    let token_url = env::var("GITHUB_TOKEN_URL").expect("oauth GITHUB_TOKEN_URL must be set");

    println!("client_id: {}", client_id);
    println!("client_secret: {}", client_secret);
    println!("redirect_url: {}", redirect_url);
    println!("auth_url: {}", auth_url);
    println!("token_url: {}", token_url);

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        Some(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
    ))
}
<<<<<<< HEAD
#[derive(Deserialize, Serialize, Debug)]
struct Plan {
    name: String,
    space: u64,
    collaborators: u64,
    private_repos: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    user_view_type: String,
    site_admin: bool,
    name: String,
    company: String,
    blog: String,
    location: String,
    email: Option<String>,
    hireable: Option<bool>,
    bio: String,
    twitter_username: Option<String>,
    notification_email: Option<String>,
    public_repos: u64,
    public_gists: u64,
    followers: u64,
    following: u64,
    created_at: String,
    updated_at: String,
    plan: Option<Plan>,
}
async fn index_handler(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(u) => format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            u.name
        ),
        None => "You're not logged in.\nVisit `/auth/discord` to do so.".to_string(),
    }
}
=======

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String
}

>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
async fn login_handler() -> impl IntoResponse {

    let client_id = env::var("GITHUB_CLIENT_ID").expect("oauth GITHUB_CLIENT_ID must be set");
    let github_redirect_url = env::var("GITHUB_REDIR_URL").expect("login GITHUB_REDIR_URL must be set");
    let state = env::var("GITHUB_STATE").expect("oauth GITHUB_STATE must be set");
<<<<<<< HEAD
    let auth_url = env::var("GITHUB_AUTH_URL").expect("oauth GITHUB_AUTH_URL must be set");
=======
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07

    println!("client_id: {}", client_id);
    println!("redirect_url: {}", github_redirect_url);
    println!("state: {}", state);

    //https://github.com/login/oauth/authorize?response_type=code&client_id=Ov23liq4S3T2Ht4KUKBR&state=repo%20user&redirect_uri=http%3A%2F%2Flocalhost%3A3005%2Fauth%2Fgithub%2Fcallback

    //let url = "https://github.com/login/oauth/authorize?response_type=code&client_id=Iv23lioYASs8e1ndFThW&state=Ih6uwwxbLv7dwxR1mRHYNYBWFmJuNA8clq0P6zqsy6k&redirect_uri=https%3A%2F%2Fopen-source-board.com%2Flogin%2Fgithub%2Fcallback";

    let github_login_url = format!(
<<<<<<< HEAD
        "{}?response_type=code&client_id={}&state={}&redirect_uri={}",
        auth_url, client_id,  encode(&state), encode(&github_redirect_url)
=======
        "https://github.com/login/oauth/authorize?response_type=code&client_id={}&state={}&redirect_uri={}",
        client_id,  encode(&state), encode(&github_redirect_url)
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
    );
    println!("github_login_url: {}", github_login_url);

    let html_content = format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Login</title>
        </head>
        <body>
            <h1>Login to GitHub</h1>
            <a href="{}">
                <button>Login with GitHub</button>
            </a>
        </body>
        </html>
    "#, github_login_url);

    Html(html_content.to_string())
}


// // callback returns code and state in query params
// async fn github_auth_handler(
//     State(client
// ): State<BasicClient>) -> impl IntoResponse {
//     // TODO: this example currently doesn't validate the CSRF token during login attempts. That
//     // makes it vulnerable to cross-site request forgery. If you copy code from this example make
//     // sure to add a check for the CSRF token.
//     //
//     // Issue for adding check to this example https://github.com/tokio-rs/axum/issues/2511
//     let (auth_url, _csrf_token) = client
//         .authorize_url(CsrfToken::new_random)
//         .add_scope(Scope::new("identify".to_string()))
//         .url();

//     // Redirect to Discord's oauth service
//     Redirect::to(auth_url.as_ref())
// }

// Valid user session required. If there is none, redirect to the auth page
async fn protected_handler(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{user:?}")
}

async fn logout_handler(
    State(store): State<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    let cookie = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(s) => s,
        // No session active, just redirect
        None => return Ok(Redirect::to("/")),
    };

    store
        .destroy_session(session)
        .await
        .context("failed to destroy session")?;

    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}
<<<<<<< HEAD
#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u32,
    refresh_token: String,
    refresh_token_expires_in: u32,
    scope: String,
    token_type: String
}

async fn exchange_code_for_token(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<AccessTokenResponse, anyhow::Error> {

    let client = reqwest::Client::new();
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
        ("redirect_uri", redirect_uri),
    ];

    println!("about to request token");

    let response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?;

    println!("finished requesting token");

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Error: {}", error_text);
        return Err(anyhow::anyhow!("Request failed with status: {} and error: {}", status, error_text));
    }

    println!("response: {:?}", response);

    // response statusCode is not 2xx throw error
    if response.status().is_success() {
        println!("response is success");

        // Read the raw response text
        let raw_response_text = response.text().await?;
        println!("Raw response text: {}", raw_response_text);

        // Deserialize the raw response text into JSON
        let token_response: AccessTokenResponse = serde_json::from_str(&raw_response_text)
            .context("failed to deserialize response as JSON")?;
        println!("token_response: {:?}", token_response);

        Ok(token_response)
    } else {
        println!("response is not success");
        let status = response.status();
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Request failed with status: {} and error: {}", status, error_text));
    }
   
}
=======
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07

async fn login_authorized_handler(
    Query(query): Query<AuthRequest>,
    State(store): State<MemoryStore>,
    State(oauth_client): State<BasicClient>,
) -> Result<impl IntoResponse, AppError> {
<<<<<<< HEAD

    println!("query: {:?}", query);

    let code = query.code.clone();
    println!("code: {:?}", code);

    let client_id = env::var("GITHUB_CLIENT_ID").expect("oauth GITHUB_CLIENT_ID must be set");
    let client_secret = env::var("GITHUB_PASSWORD").expect("oauth GITHUB_PASSWORD must be set");
    let redirect_url = env::var("GITHUB_REDIR_URL").expect("oauth GITHUB_REDIR_URL must be set");
    let auth_url = env::var("GITHUB_AUTH_URL").expect("oauth GITHUB_AUTH_URL must be set");
    let token_url = env::var("GITHUB_TOKEN_URL").expect("oauth GITHUB_TOKEN_URL must be set");

    let token = exchange_code_for_token(
        &client_id,
        &client_secret,
        &code,
        &redirect_url,
    ).await?;
    println!("token: {:?}", token);

    let user_data = fetch_user_data(&token.access_token)
        .await
        .context("failed to fetch user data")?;

    println!("user_data: {:?}", user_data);
=======
    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .context("failed in sending request request to authorization server")?;

    // Fetch user data from github
    let client = reqwest::Client::new();
    let user_data: User = client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to deserialize response as JSON")?;
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07

    // Create a new session filled with user data
    let mut session = Session::new();
    session
        .insert("user", &user_data)
        .context("failed in inserting serialized value into session")?;

<<<<<<< HEAD
    println!("session: {:?}", session);

=======
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
    // Store session and get corresponding cookie
    let cookie = store
        .store_session(session)
        .await
        .context("failed to store session")?
        .context("unexpected error retrieving cookie value")?;

<<<<<<< HEAD
    println!("cookie: {:?}", cookie);

    // Build the cookie kv pair
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

    println!("cookie format: {:?}", cookie);

=======
    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");

>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

<<<<<<< HEAD
    println!("set headers: {:?}", headers);

=======
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
    Ok((headers, Redirect::to("/")))
}

struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/github").into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}

<<<<<<< HEAD
async fn fetch_user_data(token: &str) -> Result<User, anyhow::Error> {

    println!("fetch user data");

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .bearer_auth(token)
        .header("User-Agent", "dfberry-test")
        .send()
        .await
        .context("failed in sending request to target URL")?;

    println!("response: {:?}", response);

    if response.status().is_success() {
        println!("response is success");

        // // Read the raw response text
        // let raw_response_text = response.text().await?;
        // println!("Raw response text for user: {}", raw_response_text);

        // // Deserialize the raw response text into JSON
        // let token_response: AccessTokenResponse = serde_json::from_str(&raw_response_text)
        //     .context("failed to deserialize response as JSON")?;
        // println!("token_response: {:?}", token_response);

        let user_data: User = response
        .json()
        .await
        .context("failed to deserialize response as JSON")?;

    Ok(user_data)

    } else {
        println!("response is not success");
        let status = response.status();
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Request failed with status: {} and error: {}", status, error_text));
    }

    

}


=======
>>>>>>> ad368f6fe76ba01078d19ddd8a59936111645d07
// Use anyhow, define error and enable '?'
// For a simplified example of using anyhow in axum check /examples/anyhow-error-response
#[derive(Debug)]
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
