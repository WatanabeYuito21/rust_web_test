use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    name: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "about.html")]
pub struct AboutTemplate;

pub async fn index() -> IndexTemplate {
    IndexTemplate {
        name: "watanabe".into(),
    }
}

pub async fn about() -> AboutTemplate {
    AboutTemplate
}
