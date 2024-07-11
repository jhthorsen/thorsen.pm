use pulldown_cmark::{Event, Tag, TagEnd};
use serde::Serialize;
use std::{fs, path::Path, path::PathBuf};

#[derive(Hash, Eq, PartialEq)]
enum Section {
    Metadata,
    Ingress,
    Skip,
}

#[derive(Serialize)]
pub struct Markdown {
    pub content: String,
    pub date: String,
    pub id: String,
    pub ingress: String,
    pub path: PathBuf,
    pub title: String,
}

impl Markdown {
    pub fn new_from_path(path: &Path) -> Markdown {
        let basename = path.file_name().unwrap().to_str().unwrap();

        let date = if basename.len() > 10 {
            basename[0..10].to_owned()
        } else {
            String::from("")
        };

        Markdown {
            content: String::from(""),
            date,
            id: basename.trim_end_matches(".md").to_owned(),
            ingress: String::from(""),
            path: path.to_path_buf(),
            title: basename.trim_end_matches(".md").replace("-", " "),
        }
    }

    pub fn parse(&mut self, content: String) {
        let mut section = Section::Skip;
        let parser = markdown_parser(&content.as_str());

        let parser = parser.map(|event| {
            match event {
                Event::Start(Tag::MetadataBlock(_)) => {
                    section = Section::Metadata;
                }
                Event::Start(Tag::Paragraph) => {
                    section = Section::Ingress;
                }
                Event::End(TagEnd::MetadataBlock(_)) => {
                    section = Section::Skip;
                }
                Event::End(TagEnd::Paragraph) => {
                    section = Section::Skip;
                }
                Event::Text(ref text) => {
                    if section == Section::Metadata {
                        if text.starts_with("title:") {
                            self.title = text.replace("title:", "").trim().to_owned();
                        } else if text.starts_with("date:") {
                            self.date = text.replace("date:", "").trim().to_owned();
                        }
                    } else if section == Section::Ingress {
                        if self.ingress.len() < 256 {
                            self.ingress.push_str(&text);
                        } else if !self.ingress.ends_with(".") {
                            self.ingress.push_str("...");
                        }
                    }
                }
                _ => {}
            };

            return event;
        });

        pulldown_cmark::html::push_html(&mut self.content, parser);
    }

    pub fn read(&mut self) -> bool {
        let file_content = fs::read_to_string(&self.path);
        if let Ok(file_content) = file_content {
            self.parse(file_content);
            return true;
        }

        return false;
    }
}

fn markdown_parser(text: &str) -> pulldown_cmark::Parser {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    pulldown_cmark::Parser::new_ext(&text, options)
}