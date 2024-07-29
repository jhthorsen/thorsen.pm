use actix_web::{http::header::ContentType, HttpResponse, web};
use std::collections::HashMap;

use crate::db::moment::Moment;
use crate::server_error::ServerError;
use crate::template::markdown::Markdown;

type QueryParams = HashMap<String, String>;

pub async fn get_moments(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);
    let qs = web::Query::<QueryParams>::from_query(req.query_string()).unwrap().into_inner();
    ctx.insert("query", &qs);

    let mut article = Markdown::default();
    article.scoped_css = "moments/scoped.css".to_owned();
    ctx.insert("article", &article);

    let moments = Moment::load_many(&state.db, qs).await?;
    ctx.insert("moments", &moments);

    let rendered = state.tera.render("moments/index.html", &ctx)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
