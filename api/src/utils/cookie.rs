fn get_cookie_string(req: actix_web::HttpRequest) -> Option<String> {
    let cookie_header = req.headers().get("cookie");

    match cookie_header {
        Some(cookie_header) => {
            let cookie_string = cookie_header.to_str().unwrap();
            Some(String::from(cookie_string))
        }
        None => None,
    }
}

pub fn get_cookie_value(req: actix_web::HttpRequest, target_cookie_key: &str) -> Option<String> {
    let cookie_string = get_cookie_string(req);
    match cookie_string {
        Some(cookie_string) => {
            let cookies = cookie_string.split(";").collect::<Vec<&str>>();
            cookies.iter().find_map(|cookie| {
                let cookie = cookie.trim();
                let (key, value) = cookie.split_once("=")?;
                if key == target_cookie_key {
                    Some(String::from(value))
                } else {
                    None
                }
            })
        }
        None => None,
    }
}
