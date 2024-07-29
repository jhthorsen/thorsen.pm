use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Default)]
pub struct Moment {
    pub content: String,
    pub cost: i64,
    pub created_at: Option<String>,
    pub ext_id: String,
    pub ext_url: String,
    pub img_url: String,
}

impl Moment {
    pub async fn load_many(
        pool: &SqlitePool,
        params: HashMap<String, String>,
    ) -> Result<Vec<Moment>, sqlx::Error> {
        let date = if let Some(v) = params.get("d") { v } else { "" };
        let tag = if let Some(v) = params.get("t") { v.trim_start_matches("#") } else { "" };

        let mut content = if let Some(v) = params.get("q") { v.to_string() } else { "".to_string() };
        if ! content.contains("%") {
            content = format!("%{}%", content);
        }

        let moments: Vec<Moment> = sqlx::query_as(
            r#"select
              m.content,
              m.cost,
              m.ext_id,
              m.ext_url,
              m.img_url,
              strftime('%FT%T', m.created_at) as created_at
            from moments m
            left join moment_tags t on t.moment_id = m.id
            where (length(?) < 1 or m.content like ?)
              and (length(?) < 1 or (t.kind = 'tag' and t.name = ?))
              and (
                   length(?) < 1
                or strftime('%Y', m.created_at) = ?
                or strftime('%Y-%m', m.created_at) = ?
                or strftime('%Y-%m-%d', m.created_at) = ?
              )
            group by m.id
            order by m.created_at desc
            limit 512"#,
        )
        .bind(&content)
        .bind(&content)
        .bind(&tag)
        .bind(&tag)
        .bind(&date)
        .bind(&date)
        .bind(&date)
        .bind(&date)
        .fetch_all(pool)
        .await?;

        Ok(moments)
    }
}
