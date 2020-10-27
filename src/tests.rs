#[cfg(test)]
mod tests {
    use super::super::reqs::*;
    #[test]
    fn get_status(){
        let response = send_request(Method::GET, "wd/hub/status", vec![], "").unwrap();
        assert!(response.contains("Server is running"));
    } 
}
