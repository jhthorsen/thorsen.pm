use actix_files::Files;
use actix_web::web;

use arbeidsdager::get_arbeidsdager_table;
use blog::{get_blog_index, get_blog_post};
use index::get_index;
use wildcard::get_wildcard;

mod arbeidsdager;
mod blog;
mod index;
mod wildcard;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let static_dir = std::env::var("THORSEN_STATIC_DIR").unwrap_or("./static".to_owned());

    cfg.service(web::resource("/").route(web::get().to(get_index)));
    cfg.service(web::resource("/arbeidsdager/table/{year}").route(web::get().to(get_arbeidsdager_table)));
    cfg.service(web::resource("/blog").route(web::get().to(get_blog_index)));
    cfg.service(web::resource("/blog.html").route(web::get().to(get_blog_index)));
    cfg.service(web::resource("/blog/{blog_id}").route(web::get().to(get_blog_post)));
    cfg.service(web::resource("/blog/{blog_id}.html").route(web::get().to(get_blog_post)));
    cfg.service(Files::new("/css", format!("{}/css", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/js", format!("{}/js", static_dir)).prefer_utf8(true));
    cfg.service(Files::new("/images", format!("{}/images", static_dir)).prefer_utf8(true));
    cfg.service(web::resource("/{wildcard:.*}").route(web::get().to(get_wildcard)));
}
