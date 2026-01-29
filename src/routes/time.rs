use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "partials/time.html")]
pub struct TimeTemplate {
    pub unix: u64,
    pub utc: String,
    pub jst: String,
}

pub async fn time() -> TimeTemplate {
    use std::time::SystemTime;

    let unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 簡易的な計算
    let utc_secs = unix % 86400;
    let utc_h = utc_secs / 3600;
    let utc_m = (utc_secs % 3600) / 60;
    let utc_s = utc_secs % 60;

    let jst_secs = (utc_secs + 9 * 3600) % 86400;
    let jst_h = jst_secs / 3600;
    let jst_m = (jst_secs % 3600) / 60;
    let jst_s = jst_secs % 60;

    TimeTemplate {
        unix,
        utc: format!("{:02}:{:02}:{:02}", utc_h, utc_m, utc_s),
        jst: format!("{:02}:{:02}:{:02}", jst_h, jst_m, jst_s),
    }
}
