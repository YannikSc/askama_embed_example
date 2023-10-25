use std::net::SocketAddr;

use askama::Template;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Html};
use axum::routing::get;
use axum::Router;

#[derive(askama::Template)]
#[template(path = "no_embed.html")]
struct NoEmbed;

async fn usual_inheritence() -> impl IntoResponse {
    HtmlTemplate(NoEmbed)
}


#[derive(askama::Template)]
#[template(path = "with_embed.html")]
struct WithEmbed;

async fn embedded_templates() -> impl IntoResponse {
    HtmlTemplate(WithEmbed)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/embed", get(embedded_templates))
        .route("/no_embed", get(usual_inheritence));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
