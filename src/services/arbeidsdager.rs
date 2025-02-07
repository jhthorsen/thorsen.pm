use actix_web::{http::header::ContentType, HttpResponse};
use chrono::prelude::*;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use reqwest;
use serde::{Deserialize, Serialize};
use Vec;

use crate::server_error::ServerError;

#[derive(Debug, Deserialize)]
struct HolidaysDocumentDate {
    date: NaiveDateTime,
    description: String,
}

#[derive(Debug, Deserialize)]
struct HolidaysDocument {
    // Ignoring the authenticated, timeTaken, statusCode fields
    data: Vec<HolidaysDocumentDate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum KnownHolidayKind {
    Holiday,
    Saturday,
    Sunday,
}

#[derive(Debug, Serialize)]
struct KnownHoliday {
    date: NaiveDate,
    description: String,
    kind: KnownHolidayKind,
}

async fn fetch_holidays(year: i32) -> Result<Vec<KnownHoliday>, reqwest::Error> {
    let url = format!("https://webapi.no/api/v1/holidays/{}", year);
    let holidays_doc = reqwest::get(&url).await?.json::<HolidaysDocument>().await?;

    let mut known = holidays_doc
        .data
        .iter()
        .map(|holiday| KnownHoliday {
            kind: KnownHolidayKind::Holiday,
            date: holiday.date.date(),
            description: holiday.description.to_owned(),
        })
        .collect::<Vec<KnownHoliday>>();

    let mut from = NaiveDate::from_yo_opt(year, 1).unwrap();
    let to = NaiveDate::from_yo_opt(year + 1, 1).unwrap();
    while from < to {
        if from.weekday() == Weekday::Sat {
            known.push(KnownHoliday {
                kind: KnownHolidayKind::Saturday,
                date: from.clone(),
                description: "Saturday".to_owned(),
            });
        } else if from.weekday() == Weekday::Sun {
            known.push(KnownHoliday {
                kind: KnownHolidayKind::Sunday,
                date: from.clone(),
                description: "Sunday".to_owned(),
            });
        }

        from += Duration::days(1);
    }

    known.sort_by(|a, b| a.date.cmp(&b.date));

    return Ok(known);
}

pub async fn get_arbeidsdager_table(
    req: actix_web::HttpRequest,
    year: actix_web::web::Path<i32>,
    state: actix_web::web::Data<crate::AppState>,
) -> Result<HttpResponse, ServerError> {
    let mut ctx = crate::template::template_context(&req);

    let rendered = if req.method() == actix_web::http::Method::HEAD {
        "".to_owned()
    } else {
        ctx.insert(
            "holidays".to_owned(),
            &fetch_holidays(year.into_inner()).await?,
        );
        state.tera.render("arbeidsdager/table.html", &ctx)?
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .append_header(("Cache-control", "max-age=86400"))
        .body(rendered))
}
