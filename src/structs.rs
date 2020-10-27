use serde::{Serialize,Deserialize};
use super::reqs::*;

#[derive(Serialize,Deserialize)]
struct Value{
     value: Session,
    
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
struct Session{
    sessionId: String,
}

pub struct Browser{
    session_url: String, //The session/ url for constructing other urls
    go_to_url: String, //The url to a website of the test
    timeouts_url: String,
    back_url: String,
    forward_url: String,
    refresh_url:String,
    title_url:String,
    window_url:String,
    window_handles_url:String,
    window_new_url:String,
    window_rect_url:String,
    frame_url:String,
    frame_parent_url:String,
    window_maximize_url:String,
    window_minimize_url:String,
    window_fullscreen_url:String,
    element_url:String,
    element_active_url:String,
    elements_url:String,
    source_url:String,
    execute_sync_url:String,
    execute_async_url:String,
    cookie_url:String,
    actions_url:String,
    alert_dismiss_url:String,
    alert_accept_url:String,
    alert_text_url:String,
    screenshot_url:String,
    print_page_url:String,
}

impl Browser{
    pub fn start_session(browser: &str, os:&str)->Browser{
        let req_body = serde_json::to_string(&serde_json::json!({
            "capabilities": {
                "alwaysMatch": {
                    "platformName": os
                },
                "firstMatch": [
                    {"browserName": browser}
                ]
            }
        })).unwrap();
        let headers = vec![format!("Content-Length: {}",req_body.len()+2)];
        let response = send_request(Method::POST, "wd/hub/session", headers, &req_body).unwrap();
        let resp_body = resp_body(response).unwrap();
        let val: Value = serde_json::from_str(&resp_body).unwrap();
        let sess_id = val.value.sessionId;
        //let sess_id = super::construct_sess_id(&session_id);
        Browser{
            session_url:format!("wd/hub/session/{}",sess_id),
            go_to_url: format!("wd/hub/session/{}/url",sess_id),
            timeouts_url: format!("wd/hub/session/{}/timeouts",sess_id),
            back_url: format!("wd/hub/session/{}/back",sess_id),
            forward_url: format!("wd/hub/session/{}/forward",sess_id),
            refresh_url:format!("wd/hub/session/{}/refresh",sess_id),
            title_url:format!("wd/hub/session/{}/title",sess_id),
            window_url:format!("wd/hub/session/{}/window",sess_id),
            window_handles_url:format!("wd/hub/session/{}/window/handles",sess_id),
            window_new_url:format!("wd/hub/session/{}/window/new",sess_id),
            window_rect_url:format!("wd/hub/session/{}/window/rect",sess_id),
            frame_url:format!("wd/hub/session/{}/frame",sess_id),
            frame_parent_url:format!("wd/hub/session/{}/frame/parent",sess_id),
            window_maximize_url:format!("wd/hub/session/{}/window/maximize",sess_id),
            window_minimize_url:format!("wd/hub/session/{}/window/minimize",sess_id),
            window_fullscreen_url:format!("wd/hub/session/{}/window/fullscreen",sess_id),
            element_url:format!("wd/hub/session/{}/element",sess_id),
            element_active_url:format!("wd/hub/session/{}/element/active",sess_id),
            elements_url:format!("wd/hub/session/{}/elements",sess_id),
            source_url:format!("wd/hub/session/{}/source",sess_id),
            execute_sync_url:format!("wd/hub/session/{}/execute/sync",sess_id),
            execute_async_url:format!("wd/hub/session/{}/execute/async",sess_id),
            cookie_url:format!("wd/hub/session/{}/cookie",sess_id),
            actions_url:format!("wd/hub/session/{}/actions",sess_id),
            alert_dismiss_url:format!("wd/hub/session/{}/alert/dismiss",sess_id),
            alert_accept_url:format!("wd/hub/session/{}/alert/accept",sess_id),
            alert_text_url:format!("wd/hub/session/{}/alert/text",sess_id),
            screenshot_url:format!("wd/hub/session/{}/screenshot",sess_id),
            print_page_url:format!("wd/hub/session/{}/print",sess_id),

        }
    }
    pub fn open(&self,url:&str){
        let body = format!(r#"{{"url":"{}"}}"#,url);
        send_request(Method::POST, &self.go_to_url, self.cont_length_header(&body), &body).unwrap();
    }
    pub fn close_browser(&self){
        send_request(Method::DELETE, &self.session_url, vec![], "").unwrap();
    }
    fn cont_length_header(&self,content:&str)->Vec<String>{
        vec![format!("Content-Length:{}",content.len()+2)]
    }
}

/*TODO Обернуть в функцию surf::get и surf::post, чтобы она принимала &self.___url и url(сайта)
Например:
impl Browser{
    pub fn open(&self, url::&str){
        
    }
}
*/
pub struct Element{
    element_id: String,
}

pub struct Cookie{
    cookie_name:String,
}





//TESTS

    #[test]
    fn create_browser_struct() {
        let browser = Browser::start_session("chrome", "windows");
        let mut iter = browser.session_url.split("/");
        iter.next();
        iter.next();
        iter.next();
        let sess = iter.next().unwrap();
        assert_eq!(32,sess.len());
    }
    #[test]
    fn go_to_bash() {
        let browser = Browser::start_session("chrome", "windows");
        browser.open("https://bash.im");
    }
    #[test]
    fn close_browser() {
        let browser = Browser::start_session("chrome", "windows");
        browser.open("https://bash.im");
        browser.close_browser();
    }
