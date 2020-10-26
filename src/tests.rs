#[cfg(test)]
mod tests {
    use super::super::reqs::*;

    #[test]
    fn start_session() {
        let temp = serde_json::json!({
            "capabilities": {
                "alwaysMatch": {
                    "platformName": "linux"
                },
                "firstMatch": [
                    {"browserName": "chrome"}
                ]
            }
        });
        let body = serde_json::to_string(&temp).unwrap();
        println!("{}", body);
        let length = body.len();
        let headers = vec![format!("Content-Length:{}", length + 2)];
        send_request(Method::POST, "wd/hub/session", headers, &body);
    }
}
