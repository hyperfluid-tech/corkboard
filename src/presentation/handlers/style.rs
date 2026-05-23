use axum::response::IntoResponse;

pub async fn style_handler() -> impl IntoResponse {
    let css = include_str!("../../../templates/style.css");
    (
        [(axum::http::header::CONTENT_TYPE, "text/css")],
        css,
    )
}
