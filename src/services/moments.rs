use actix_web::{http::header::ContentType, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

use crate::server_error::ServerError;
use crate::template::markdown::Markdown;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMomentsQuery {
    pub d: Option<String>,
    pub q: Option<String>,
    pub t: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Moment {
    pub content: String,
    pub cost: i64,
    pub created_at: Option<String>,
    pub ext_id: String,
    pub ext_url: String,
    pub img_url: String,
}

pub async fn get_moments(
    web_query: actix_web::web::Query<GetMomentsQuery>,
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ServerError> {
    let web_query = web_query.into_inner();

    let mut article = Markdown::default();
    article.scoped_css = "moments/scoped.css".to_owned();

    let mut ctx = crate::template::template_context(&req);
    ctx.insert("article", &article);

    let mut and: Vec<String> = Vec::new();
    let mut from: Vec<String> = Vec::new();
    let mut bind: Vec<String> = Vec::new();

    from.push("moments m".to_owned());

    if let Some(v) = &web_query.t {
        if v.len() > 0 {
            from.push("left join moment_tags t on t.moment_id = m.id".to_owned());
            and.push("t.kind = 'tag' and t.name = ?".to_owned());
            bind.push(v.to_owned());
        }
    }
    if let Some(v) = &web_query.d {
        if v.len() > 0 {
            and.push("date(created_at) = ?".to_owned());
            bind.push(v.to_owned());
        }
    }
    if let Some(v) = &web_query.q {
        if v.len() > 0 {
            and.push("content like ?".to_owned());
            bind.push(format!("%{}%", v.to_owned()));
        }
    }
    if and.len() == 0 {
        and.push("1 = 1".to_owned());
    }

    let sst = format!(
        "select
        content,
        cost,
        ext_id,
        ext_url,
        img_url,
        strftime('%FT%T', created_at) as created_at
        from {}
        where {}
        order by created_at desc",
        from.join(" "),
        and.join(" and "),
    );

    //println!("{} // {:?} // {:?}", sst, bind, web_query);

    let mut db_query = sqlx::query(sst.as_str());
    for v in bind {
        db_query = db_query.bind(v);
    }

    let moments = db_query
        .map(|row: SqliteRow| Moment {
            content: row.get::<String, _>("content"),
            cost: row.get::<i64, _>("cost"),
            created_at: row.get::<Option<String>, _>("created_at"),
            ext_id: row.get::<String, _>("ext_id"),
            ext_url: row.get::<String, _>("ext_url"),
            img_url: row.get::<String, _>("img_url"),
        })
        .fetch_all(&state.db)
        .await?;

    ctx.insert("query", &web_query);
    ctx.insert("moments", &moments);

    let rendered = state.tera.render("moments/index.html", &ctx)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
