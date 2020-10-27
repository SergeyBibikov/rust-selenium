#[cfg(test)]
mod tests {
    use super::super::reqs::*;
    #[cfg(target_os = "linux")]
    #[test]
    fn session_id_linux() {
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
        //println!("{}", body);
        let length = body.len();
        let headers = vec![format!("Content-Length:{}", length + 2)];
        let response = send_request(Method::POST, "wd/hub/session", headers, &body).unwrap();
        assert!(response.contains("sessionId"));
    }
    #[cfg(target_os = "windows")]
    #[test]
    fn session_id_windows() {
        let temp = serde_json::json!({
            "capabilities": {
                "alwaysMatch": {
                    "platformName": "windows"
                },
                "firstMatch": [
                    {"browserName": "chrome"}
                ]
            }
        });
        let body = serde_json::to_string(&temp).unwrap();
        //println!("{}", body);
        let length = body.len();
        let headers = vec![format!("Content-Length:{}", length + 2)];
        let response = send_request(Method::POST, "wd/hub/session", headers, &body).unwrap();
        assert!(response.contains("sessionId"));
    }
    #[test]
    fn get_status(){
        let response = send_request(Method::GET, "wd/hub/status", vec![], "").unwrap();
        assert!(response.contains("Server is running"));
    }
}
