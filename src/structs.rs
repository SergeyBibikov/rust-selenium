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
    pub fn get_window_handle(&self)->String{
        let resp = send_and_read_body(Method::GET, &self.window_url, vec![], "");
        let map:HashMap<&str,&str> = serde_json::from_str(&resp).unwrap();
        (map.get("value").unwrap()).to_string().clone()
    }
    pub fn get_window_handles(&self)->Vec<String>{
       let mut result:Vec<String>=vec![];
       let resp = send_and_read_body(Method::GET, &self.window_handles_url, vec![], "");
       let map:HashMap<&str,Vec<String>> = serde_json::from_str(&resp).unwrap();
       for i in map.get(&"value").unwrap(){
        result.push(i.to_owned());
       }
       result
    }
    pub fn switch_to_window(&self, window_id: String)->Result<(),String>{
        let body = format!(r#"{{"handle":"{}"}}"#,window_id);
        let resp = send_and_read_body(Method::POST, &self.window_url, self.cont_length_header(&body), &body);
        if resp.as_str()==r#"{"value":null}"#{
            Ok(())
        }else{Err(resp)}
    }
    pub fn new_window(&self, window_type: NewWindowType)->(String,String){
        let body = match window_type{
            NewWindowType::Tab=>r#"{"type":"tab"}"#,
            NewWindowType::Window=>r#"{"type":"window"}"#,
        };
        let resp=send_and_read_body(Method::POST, &self.window_new_url, self.cont_length_header(&body), &body);
        println!("{}",resp);
        let map: HashMap<&str,HashMap<&str,String>> = serde_json::from_str(&resp).unwrap();
        let val = map.get(&"value").unwrap();
        let handle = (*val).get("handle").unwrap().clone();
        let wtype = (*val).get("type").unwrap().clone();
        (handle,wtype)
    }
    pub fn close_window(&self)->Vec<String>{
       let mut result:Vec<String>=vec![];
       let resp = send_and_read_body(Method::DELETE, &self.window_url, vec![], "");
       let map:HashMap<&str,Vec<String>> = serde_json::from_str(&resp).unwrap();
       for i in map.get(&"value").unwrap(){
        result.push(i.to_owned());
       }
       result        
    }
    pub fn switch_to_frame_by_id(&self, id: u64)->Result<(),String>{
        let body = format!(r#"{{"id":{}}}"#,id);
        let resp = send_and_read_body(Method::POST, &self.frame_url, self.cont_length_header(&body), &body);
        if resp.as_str()!=r#"{"value":null}"#{
            return Err(resp);
        }
        Ok(())
    }
    pub fn switch_to_frame_by_element(&self, element:Element)->Result<(),String>{
        let body = format!(r#"{{"id":{{"{}":"{}"}}}}"#,element.element_id,element.element_hash);
        let resp = send_and_read_body(Method::POST, &self.frame_url, self.cont_length_header(&body), &body);
        if resp.as_str()!=r#"{"value":null}"#{
            return Err(resp);
        }
        Ok(())

    }
    pub fn switch_to_parent_frame(&self)->Result<(),String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.frame_parent_url, self.cont_length_header(&body), &body);
        if resp.as_str()!=r#"{"value":null}"#{
            return Err(resp);
        }
        Ok(())
    }
    pub fn find_element(&self,loc_strategy:LocStrategy)->Element{
        let body = self.body_for_find_element(loc_strategy);
        let resp=send_and_read_body(Method::POST, &self.element_url, self.cont_length_header(&body), &body);
        let map: HashMap<&str,HashMap<String,String>> = serde_json::from_str(&resp).unwrap();
        let val = map.get(&"value").unwrap();
        let res = val.iter().next().unwrap();
        Element{
            element_id:res.0.clone(),
            element_hash:res.1.clone(),
            element_url: format!("{}/element",self.session_url),
        }
    }
    pub fn find_elements(&self,loc_strategy:LocStrategy)->Vec<Element>{
        let mut result = vec![];
        let body = self.body_for_find_element(loc_strategy);
        let resp=send_and_read_body(Method::POST, &self.elements_url, self.cont_length_header(&body), &body);
        let map: HashMap<&str,Vec<HashMap<String,String>>> = serde_json::from_str(&resp).unwrap();
        let val = map.get(&"value").unwrap();
        let element_url = format!("{}/element",self.session_url);
        for i in val{
            let element_url = element_url.clone();
            let res = i.iter().next().unwrap();
            result.push(Element{
            element_id:res.0.clone(),
            element_hash:res.1.clone(),
            element_url
            });
        }
        result
    }
    pub fn get_window_rect(&self)->WindowRect{
        let resp = send_and_read_body(Method::GET, &self.window_rect_url, vec![], "");
        let map:HashMap<&str,WindowRect> = serde_json::from_str(&resp).unwrap();
        map.get("value").unwrap().clone()
    }
    pub fn set_sindow_rect(&self, window_rect:&WindowRect)->Result<WindowRect,String>{
        let body = serde_json::to_string(window_rect).unwrap();
        let resp = send_and_read_body(Method::POST, &self.window_rect_url, self.cont_length_header(&body), &body);
        let map:Result<HashMap<&str,WindowRect>,serde_json::Error> = serde_json::from_str(&resp);
        match map{
            Ok(cont)=>{
                Ok(cont.get("value").unwrap().clone())
            },
            Err(message)=>Err(message.to_string()),
        }
                
    }
    pub fn maximize_window(&self)->Result<WindowRect,String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.window_maximize_url, self.cont_length_header(&body), &body);
        if resp.contains("height")&&resp.contains("width"){
            let map:HashMap<&str,WindowRect> = serde_json::from_str(&resp).unwrap();
            Ok(map.get("value").unwrap().clone())
        }else{Err(resp)}

    }
    pub fn minimize_window(&self)->Result<WindowRect,String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.window_minimize_url, self.cont_length_header(&body), &body);
        if resp.contains("height")&&resp.contains("width"){
            let map:HashMap<&str,WindowRect> = serde_json::from_str(&resp).unwrap();
            Ok(map.get("value").unwrap().clone())
        }else{Err(resp)}
    } 



    fn cont_length_header(&self,content:&str)->Vec<String>{
        vec![format!("Content-Length:{}",content.len()+2)]
    }
    fn body_for_find_element(&self,loc_strategy:LocStrategy)->String{
        match loc_strategy{
            LocStrategy::CSS(selector)=>format!(r#"{{"using":"css selector","value":"{}"}}"#,selector),
            LocStrategy::LINKTEXT(selector)=>format!(r#"{{"using":"link text","value":"{}"}}"#,selector),
            LocStrategy::PARTLINKTEXT(selector)=>format!(r#"{{"using":"partial link text","value":"{}"}}"#,selector),
            LocStrategy::TAGNAME(selector)=>format!(r#"{{"using":"tag name","value":"{}"}}"#,selector),
            LocStrategy::XPATH(selector)=>format!(r#"{{"using":"xpath","value":"{}"}}"#,selector)
        }
    }

    
}
    fn send_and_read_body(method: Method, path: &str, headers: Vec<String>, body: &str)->String{
        resp_body(send_request(method, path, headers, body).unwrap()).unwrap()
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
            two
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
pub enum NewWindowType{
    Tab,
    Window
}
pub enum LocStrategy{
    CSS(&'static str),
    LINKTEXT(&'static str),
    PARTLINKTEXT(&'static str),
    TAGNAME(&'static str),
    XPATH(&'static str)
}

#[derive(Debug)]
pub struct Element{
    pub(self)element_id: String,
    pub(self)element_hash: String,
    pub(self)element_url: String,
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Clone)]
pub struct WindowRect{
    pub(self)height:i32,
    pub(self)width:i32,
    pub(self)x:i32,
    pub(self)y:i32
}

impl WindowRect{
    pub fn new(height:i32,width:i32,x:i32,y:i32)->WindowRect{
        WindowRect{ height, width, x, y}
    }
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
    fn go_to_vk() {
        let mut browser = Browser::start_session("chrome", consts::OS,vec!["--headless"]);
        browser.open("https://vk.com");
        let link= browser.get_link();
        browser.close_browser();
        assert_eq!(link.as_str(),"https://vk.com/");
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
        let link: String;
        {let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com/");
        br.open("https://m.facebook.com/");
        br.back();
        link = br.get_link();
        br.close_browser();}
        assert_eq!(link.as_str(),"https://vk.com/");
    }
    #[test]
    fn forward_test() {
        let link: String;
        {let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com/");
        br.open("https://m.facebook.com/");
        br.back();
        br.forward();
        link = br.get_link();        
        br.close_browser();}
        assert_eq!(link.as_str(),"https://m.facebook.com/");
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
        let title:String;
        {let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://www.w3.org/TR/webdriver/");
        title =br.get_title();
        br.close_browser()}
        assert_eq!(title,String::from("WebDriver"));
        
    }
    #[test]
    fn window_handle() {
        let handle: String;
        {let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        handle = br.get_window_handle();
        br.close_browser();}
        assert!(handle.starts_with("CDwindow"));
        
    }
    #[test]
    fn switch_window(){
        let res :Result<(),String>;
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let handle = br.get_window_handle();
        res = br.switch_to_window(handle);assert_eq!(Ok(()),res);
        br.close_browser();
    }
    #[test]
    fn get_handles() {
        let handles;
        {
            let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
            br.open("https://vk.com");
            handles = br.get_window_handles();
            br.close_browser();
        }
        assert_eq!(handles.len(),1);
    }
    #[test]
    fn close_window() {
        let handles;
        {
            let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
            br.open("https://vk.com");
            handles = br.close_window();
            br.close_browser();
        }
        assert_eq!(handles.len(),0);
    }
    #[test]
    fn new_window(){
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        let wind = br.new_window(NewWindowType::Tab);
        assert!(wind.1=="tab"&&br.get_window_handles().len()==2);
        br.close_browser();
    }
    #[test]
    fn sw_to_frame_by_id() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://bash.im");
        let res = br.switch_to_frame_by_id(0);
        br.close_browser();
        assert_eq!(res,Ok(()));

    }
    #[test]
    fn sw_t_fr_by_el() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let el = br.find_element(LocStrategy::CSS("#quick_login_frame"));
        dbg!(&el);
        let res = br.switch_to_frame_by_element(el);
        br.close_browser();
        assert_eq!(res,Ok(()));
    }
    #[test]
    fn find_element() {
        let el;
        {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://bash.im");
        el = br.find_element(LocStrategy::CSS("#criteo-syncframe"));
        br.close_browser();
        }
        assert!(el.element_id.contains("element"))
    }
    #[test]
    fn find_els() {
        let el;
        {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://bash.im");
        el = br.find_elements(LocStrategy::CSS("article"));
        br.close_browser();
        }
        assert!(el.len()>2);
    }
    #[test]
    fn sw_to_par_fr() {
        let res;
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        res = br.switch_to_parent_frame();
        br.close_browser(); 
        assert_eq!(res, Ok(()));
    }
    #[test]
    fn get_wind_rect() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        let wr = br.get_window_rect();
        br.close_browser();
        assert!(wr==WindowRect{height:200,width:400,x:0,y:0});
    }
    #[test]
    fn set_wind_rect(){
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        let wr = WindowRect{height:600, width:1000,x:250,y:250};
        let wr_new = br.set_sindow_rect(&wr).unwrap();
        br.close_browser();
        assert_eq!(wr,wr_new);
        
    }
    #[test]
    fn maximize() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        let a = br.maximize_window().unwrap();
        br.close_browser();
        let wr = WindowRect{height:200, width:400,x:0,y:0};
        assert_eq!(a,wr);
    }

}