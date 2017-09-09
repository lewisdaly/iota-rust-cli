

pub fn format_url(protocol: String, host: String, port: i32, path: String) -> String {
    let mut url = "".to_owned();

    url.push_str(&protocol);
    url.push_str(&host);
    url.push_str(":");
    url.push_str(&port.to_string());
    url.push_str(&path);
    url
}
