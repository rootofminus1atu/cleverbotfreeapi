use chrono::{DateTime, Utc};

pub async fn get_cookie(client: &reqwest::Client) -> Result<String, crate::Error> {
    let url = format!("https://www.cleverbot.com/extras/conversation-social-min.js?{}", get_date());
    let resp = client.get(&url).send().await?;

    let cookie_before = resp.headers()
        .get("set-cookie")
        .and_then(|s| s.to_str().ok())
        .and_then(|s| s.split(';').next());

    let cookie_str = cookie_before
        .map(|s| s.replace("B%", "32"));  // i have no idea why 31 ore 32 work, but other ones don't

    cookie_str.ok_or(crate::Error::NoCookieFound)
}

fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y%m%d").to_string()
}
