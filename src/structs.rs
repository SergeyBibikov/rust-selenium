use serde::{Serialize,Deserialize};
use super::reqs::*;
use std::collections::HashMap;

#[derive(Serialize,Deserialize)]
struct Value{
     value: Session,
    
}
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize)]
struct Session{
    sessionId: String,
}
#[derive(Debug)]
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
    pub fn start_session(browser: &str, os:&str,args:Vec<&str>)->Browser{
        let req_body = create_session_body_json(browser,os,args);
        let headers = vec![format!("Content-Length: {}",req_body.len()+2)];
        let response = send_request(Method::POST, "wd/hub/session", headers, &req_body).unwrap();
        let resp_body = resp_body(response).unwrap();
        let val: Value = serde_json::from_str(&resp_body).unwrap();
        let sess_id = val.value.sessionId;
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
    pub fn get_link(&self)->String{
        let resp = resp_body(send_request(Method::GET, &self.go_to_url, vec![], "").unwrap()).unwrap();
        let temp_map:HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        let a = temp_map.get("value").unwrap();
        (*a).clone()
    }
    pub fn close_browser(&mut self){
        send_request(Method::DELETE, &self.session_url, vec![], "").unwrap();
        self.session_url = String::from("");
    }
    pub fn get_timeouts(&self)->String{
        resp_body(send_request(Method::GET, &self.timeouts_url, vec![], "").unwrap()).unwrap()
    }
    pub fn set_timeouts(&self, timeouts: &Timeouts)->Result<(),&str>{
        let timeouts_json = serde_json::to_string(timeouts).unwrap();
        if let Ok(mess)= send_request(Method::POST, &self.timeouts_url, self.cont_length_header(&timeouts_json), &timeouts_json){
            if let Ok (body) = resp_body(mess){
                if body.as_str() == r#"{"value":null}"#{
                    return Ok(())
                }
            }
        }
        Err("The timeouts were not set correctly")
    }
    pub fn back(&self){
        let body = r#"{"return":true}"#;
        send_request(Method::POST,&self.back_url, self.cont_length_header(&body), &body).unwrap();
    }
    pub fn forward(&self){
        let body = r#"{"forward":true}"#;
        send_request(Method::POST,&self.forward_url, self.cont_length_header(&body), &body).unwrap();
    }
    pub fn refresh(&self)->Result<(),&str>{
        let body = r#"{"refresh":true}"#;
        if let Ok (mess) = send_request(Method::POST,&self.refresh_url, self.cont_length_header(&body), &body){
            if let Ok (body) = resp_body(mess){
                if body.as_str()!=r#"{"value":null}"#{
                   return Err("The refresh did not succeed")
                }
            }
        }
        Ok(())
    }
    pub fn get_title(&self)->String{
        let json = resp_body(send_request(Method::GET, &self.title_url, vec![], "").unwrap()).unwrap();
        let val:HashMap<&str,String> = serde_json::from_str(&json).unwrap();
        let title = val.get("value").unwrap(); 
        (*title).clone()
    }

    fn cont_length_header(&self,content:&str)->Vec<String>{
        vec![format!("Content-Length:{}",content.len()+2)]
    }
}

    fn create_session_body_json(browser:&str,os:&str, args:Vec<&str>)->String{
            match browser{
                "chrome"=> create_chrome_session(os,args),
                _=>panic!("Sorry, so far only chrome is supported")
            }
    }
    fn create_chrome_session(os:&str,args:Vec<&str>)->String{
            let one=format!(r#"{{"capabilities": {{"alwaysMatch":{{"platformName":"{}"}}"#,os);
            let args = gen_args(args);
            let two=format!(r#"{},"firstMatch":[{{"browserName":"chrome","goog:chromeOptions":{{"args":{}}}}}]}}}}"#,one,args);
            two//String::new()
    }

    fn gen_args(args:Vec<&str>)->String{
            if args.len()==0{
                return String::from("[]");
            }
            let mut result = String::from("[");
            for arg in args{
                result.push_str("\"");
                result.push_str(arg);
                result.push_str("\"");
                result.push_str(",");
            }
            result.pop();
            result.push_str("]");
            result
    }
/*
TODO
pub struct ChromeOptions{}
*/

pub struct Element{
    element_id: String,
}

pub struct Cookie{
    cookie_name:String,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub struct Timeouts{
    implicit:u32,
    pageLoad:u32,
    script:u32,
}
impl Timeouts{
    ///Instantiates the Timeouts with all fields set
    pub fn set_all (implicit: u32, page_load: u32,script:u32)->Timeouts{
        Timeouts{
            implicit,
            pageLoad: page_load,
            script
        }
    }
    ///Instantiates the Timeouts with all fields == 0
    pub fn new ()->Timeouts{
        Timeouts{
            implicit:0,
            pageLoad: 0,
            script:0,
        }
    }
    pub fn set_implicit(&mut self,implicit:u32){
        self.implicit=implicit;
    }
    pub fn set_page_load(&mut self,page_load:u32){
        self.pageLoad=page_load;
    }
    pub fn set_script(&mut self,script:u32){
        self.script=script;
    }

}


//TESTS

pub mod tests{
    use super::*;
    use std::env::*;
    #[test]
    fn create_session() {
        let mut browser = Browser::start_session("chrome", consts::OS,vec!["--headless"]);
        let sess:String; 
        {let mut iter = browser.session_url.split("/");
        iter.next();
        iter.next();
        iter.next();
        sess = iter.next().unwrap().to_string();}
        browser.close_browser();
        
        assert_eq!(32,sess.len());
    }
    #[test]
    fn go_to_bash() {
        let mut browser = Browser::start_session("chrome", consts::OS,vec!["--headless"]);
        browser.open("https://bash.im");
        let link= browser.get_link();
        browser.close_browser();
        assert_eq!(link.as_str(),"https://bash.im/");
    }
    #[test]
    #[should_panic]
    fn close_browser() {
        let mut browser = Browser::start_session("chrome", consts::OS,vec!["--headless"]);
        browser.open("http://localhost:4444/wd/hub/status");
        browser.close_browser();
        browser.get_link();
    }
    #[test]
    fn args() {
        let a = vec!["--headless","--window-size=800,400"];        
        assert!(gen_args(a)==String::from("[\"--headless\",\"--window-size=800,400\"]"));
    }
    #[test]
    fn get_timeouts() {
        let timeouts:String;
        {
            let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
            timeouts= br.get_timeouts();
            br.close_browser();
        }
        assert!(timeouts.contains("value")&&timeouts.contains("implicit")&&timeouts.contains("pageLoad"));
    }

    #[test]
    fn set_timeouts() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        let timeouts = Timeouts::set_all(1000, 3000, 300000);
        assert!(br.set_timeouts(&timeouts)==Ok(()));
        br.close_browser();
    }
    #[test]
    fn check_timouts_init() {
        let mut t = Timeouts::new();
        assert_eq!(t,Timeouts{implicit:0,pageLoad:0,script:0});
        t.set_implicit(1);
        assert_eq!(t,Timeouts{implicit:1,pageLoad:0,script:0});
        t.set_page_load(1);
        assert_eq!(t,Timeouts{implicit:1,pageLoad:1,script:0});
        t.set_script(1);
        assert_eq!(t,Timeouts{implicit:1,pageLoad:1,script:1});
    }
    #[test]
    fn back_test() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com/");
        br.open("https://bash.im/");
        br.back();
        assert_eq!(br.get_link(),String::from("https://vk.com/"));
        br.close_browser()
    }
    #[test]
    fn forward_test() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com/");
        br.open("https://bash.im/");
        br.back();
        br.forward();
        assert_eq!(br.get_link(),String::from("https://bash.im/"));
        br.close_browser()
    }
    #[test]
    fn refresh_test() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com/");
        assert_eq!(br.refresh(),Ok(()));
        br.close_browser()
    }
    #[test]
    fn get_title_test() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://www.w3.org/TR/webdriver/");
        assert_eq!(br.get_title(),String::from("WebDriver"));
        br.close_browser()
    }

}