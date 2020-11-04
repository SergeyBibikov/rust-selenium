extern crate base64;

use serde::{Serialize,Deserialize};
use super::reqs::*;
use std::collections::HashMap;
use self::utils::*;
use super::element::*;

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
        send_request(Method::POST, &self.go_to_url, cont_length_header(&body), &body).unwrap();
    }
    pub fn get_link(&self)->String{
        let resp = resp_body(send_request(Method::GET, &self.go_to_url, vec![], "").unwrap()).unwrap();
        dbg!(&resp);
        parse_value(&resp).replace("\"","")
    }
    ///Deletes current session. As the Browser struct needs dropping, it is better to call drop() after this method in long-multibrowser scenarios.  
    pub fn close_browser(&mut self){
        send_request(Method::DELETE, &self.session_url, vec![], "").unwrap();
        self.session_url = String::from("");
    }
    pub fn get_timeouts(&self)->Timeouts{
        let resp = send_and_read_body(Method::GET, &self.timeouts_url, vec![], "");
        serde_json::from_str(&parse_value(&resp)).unwrap()
    }
    pub fn set_timeouts(&self, timeouts: &Timeouts)->Result<(),&str>{
        let timeouts_json = serde_json::to_string(timeouts).unwrap();
        if let Ok(mess)= send_request(Method::POST, &self.timeouts_url, cont_length_header(&timeouts_json), &timeouts_json){
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
        send_request(Method::POST,&self.back_url, cont_length_header(&body), &body).unwrap();
    }
    pub fn forward(&self){
        let body = r#"{"forward":true}"#;
        send_request(Method::POST,&self.forward_url, cont_length_header(&body), &body).unwrap();
    }
    pub fn refresh(&self)->Result<(),&str>{
        let body = r#"{"refresh":true}"#;
        if let Ok (mess) = send_request(Method::POST,&self.refresh_url, cont_length_header(&body), &body){
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
        parse_value(&json).replace("\"","")
    }
    pub fn get_window_handle(&self)->String{
        let resp = send_and_read_body(Method::GET, &self.window_url, vec![], "");
        parse_value(&resp).replace("\"","")
        /*let map:HashMap<&str,&str> = serde_json::from_str(&resp).unwrap();
        (map.get("value").unwrap()).to_string().clone()*/
    }
    pub fn get_window_handles(&self)->Vec<String>{
       let resp = send_and_read_body(Method::GET, &self.window_handles_url, vec![], "");
       let resp = parse_value(&resp);
       let res:Vec<String> = serde_json::from_str(&resp).unwrap();
       res
    }
    pub fn switch_to_window(&self, window_id: String)->Result<(),String>{
        let body = format!(r#"{{"handle":"{}"}}"#,window_id);
        let resp = send_and_read_body(Method::POST, &self.window_url, cont_length_header(&body), &body);
        if resp.as_str()==r#"{"value":null}"#{
            Ok(())
        }else{Err(resp)}
    }
    pub fn new_window(&self, window_type: NewWindowType)->(String,String){
        let body = match window_type{
            NewWindowType::Tab=>r#"{"type":"tab"}"#,
            NewWindowType::Window=>r#"{"type":"window"}"#,
        };
        let resp=send_and_read_body(Method::POST, &self.window_new_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        let map: HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        let handle = map.get("handle").unwrap().clone();
        let wtype = map.get("type").unwrap().clone();
        (handle,wtype)
    }
    pub fn close_window(&self)->Vec<String>{
       let resp = send_and_read_body(Method::DELETE, &self.window_url, vec![], "");
       let resp = parse_value(&resp);
       let res:Vec<String> = serde_json::from_str(&resp).unwrap();
       res        
    }
    pub fn switch_to_frame_by_id(&self, id: u64)->Result<(),String>{
        let body = format!(r#"{{"id":{}}}"#,id);
        let resp = send_and_read_body(Method::POST, &self.frame_url, cont_length_header(&body), &body);
        if resp.as_str()!=r#"{"value":null}"#{
            return Err(resp);
        }
        Ok(())
    }
    pub fn switch_to_frame_by_element(&self, element:Element)->Result<(),String>{
        let body = format!(r#"{{"id":{{"{}":"{}"}}}}"#,element.element_gr_id,element.element_id);
        let resp = send_and_read_body(Method::POST, &self.frame_url, cont_length_header(&body), &body);
        if resp.as_str()!=r#"{"value":null}"#{
            return Err(resp);
        }
        Ok(())

    }
    pub fn switch_to_parent_frame(&self)->Result<(),String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.frame_parent_url, cont_length_header(&body), &body);
        if resp.as_str()!=r#"{"value":null}"#{
            return Err(resp);
        }
        Ok(())
    }
    pub fn get_active_element(&self)->Result<Element,String>{
        let resp = send_and_read_body(Method::GET, &self.element_active_url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let resp = parse_value(&resp);
        let map: HashMap<String,String> = serde_json::from_str(&resp).unwrap();
        let res = map.iter().next().unwrap();
        Ok(Element{
            element_gr_id:res.0.clone(),
            element_id:res.1.clone(),
            element_url: format!("{}/element/{}",self.session_url,res.1.clone()),
        })
    }
    pub fn find_element(&self,loc_strategy:LocatorStrategy)->Element{
        let body = body_for_find_element(loc_strategy);
        let resp=send_and_read_body(Method::POST, &self.element_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        let map: HashMap<String,String> = serde_json::from_str(&resp).unwrap();
        let res = map.iter().next().unwrap();
        Element{
            element_gr_id:res.0.clone(),
            element_id:res.1.clone(),
            element_url: format!("{}/element/{}",self.session_url,res.1.clone()),
        }
    }
    pub fn find_elements(&self,loc_strategy:LocatorStrategy)->Vec<Element>{
        let mut result = vec![];
        let body = body_for_find_element(loc_strategy);
        let resp=send_and_read_body(Method::POST, &self.elements_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        let map: Vec<HashMap<String,String>> = serde_json::from_str(&resp).unwrap();
        let element_ur = format!("{}/element",self.session_url);
        for i in map{
            let element_ur = element_ur.clone();
            let res = i.iter().next().unwrap();
            result.push(Element{
            element_gr_id:res.0.clone(),
            element_id:res.1.clone(),
            element_url:format!("{}/{}",element_ur,res.1.clone()),
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
        let resp = send_and_read_body(Method::POST, &self.window_rect_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        let map:Result<WindowRect,serde_json::Error> = serde_json::from_str(&resp);
        match map{
            Ok(cont)=>{
                Ok(cont)
            },
            Err(message)=>Err(message.to_string()),
        }
                
    }
    pub fn maximize_window(&self)->Result<WindowRect,String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.window_maximize_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        if resp.contains("height")&&resp.contains("width"){
            let res: WindowRect = serde_json::from_str(&resp).unwrap();
            Ok(res)
        }else{Err(resp)}

    }
    pub fn minimize_window(&self)->Result<WindowRect,String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.window_minimize_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        if resp.contains("height")&&resp.contains("width"){
            let res: WindowRect = serde_json::from_str(&resp).unwrap();
            Ok(res)
        }else{Err(resp)}
    }
    pub fn fullscreen(&self)->Result<WindowRect,String>{
        let body = r#"{}"#;
        let resp = send_and_read_body(Method::POST, &self.window_fullscreen_url, cont_length_header(&body), &body);
        let resp = parse_value(&resp);
        if resp.contains("height"){
            Ok(serde_json::from_str(&resp).unwrap())
        } else {Err(resp)}
    }
    pub fn source(&self)->String{
        let resp = send_and_read_body(Method::GET, &self.source_url, vec![], "");
        let map:HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        map.get("value").unwrap().clone()
    }
    pub fn get_all_cookies(&self)->Vec<Cookie>{
        let mut result: Vec<Cookie> = vec![];
        let resp = send_and_read_body(Method::GET, &self.cookie_url, vec![], "");
        let map:HashMap<&str,Vec<serde_json::Value>> = serde_json::from_str(&resp).unwrap();
        for v in map.get("value").unwrap(){
            result.push(from_value_to_cookie(v));
        }  
        result   
    }
    pub fn get_cookie(&self,cookie_name:&str)->Result<Cookie,String>{
        let url = format!("{}/{}",self.cookie_url,cookie_name);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("domain")&&resp.contains("expiry")&&resp.contains("name"){
            let map:HashMap<&str,serde_json::Value> = serde_json::from_str(&resp).unwrap();
            let v = map.get(&"value").unwrap();
            return Ok(from_value_to_cookie(v));
        }
        Err(resp)
    }
    pub fn add_cookie(&self,cookie:Cookie)->Result<(),String>{
        let cook = serde_json::to_string(&cookie).unwrap();
        let body =format!(r#"{{"cookie": {} }}"#,cook);
        let resp = send_and_read_body(Method::POST, &self.cookie_url, cont_length_header(&body), &body);
        if resp==r#"{"value":null}"#{
            return Ok(());
        }
        Err(resp)
    }
    pub fn delete_cookie(&self,cookie_name:&str)->Result<(),String>{
        let uri = format!("{}/{}",self.cookie_url,cookie_name);
        let resp = send_and_read_body(Method::DELETE, &uri, vec![], "");
        if resp==r#"{"value":null}"#{
            return Ok(());
        }
        Err(resp)
    }
    pub fn delete_all_cookies(&self)->Result<(),String>{
        let resp = send_and_read_body(Method::DELETE, &self.cookie_url, vec![], "");
        if resp==r#"{"value":null}"#{
            return Ok(());
        }
        Err(resp)
    }
    pub fn take_screenshot(&self,path:&str)->Result<(),String>{
        if let Ok(resp) = send_request_screensh(Method::GET, &self.screenshot_url, vec![], ""){
             if let Ok(new) = base64::decode(resp){
                match std::fs::write(path,new){
                    Ok(())=>return Ok(()),
                    Err(message)=> return Err(message.to_string()),
                }
             }
        }
        Err(String::from("Could not take a screenshot"))     
    }
    pub fn take_element_screenshot(&self,elem:&Element,path: &str)->Result<(),String>{
        let uri = format!("{}/{}/screenshot",self.element_url,elem.element_id);
        if let Ok(resp) = send_request_screensh(Method::GET, &uri, vec![], ""){
            if let Ok(new) = base64::decode(resp){
                match std::fs::write(path,new){
                    Ok(())=>return Ok(()),
                    Err(message)=>return Err(message.to_string())
                }
            }
        }
        Err(String::from("Could not take a screenshot")) 
    }
    /// Executes the sync fun in the browser. In case the argument is a string, it should be a raw string or should incluse escapes with double quotes
    /// For example, if the args list you want to pass is [5,"Jack", 15], the vector should be ["5",r#"Jack"#,"15"]
    pub fn execute_sync(&self, script: &str, args: &Vec<&str>)->Result<String,String>{
        let args = gen_script_args(args);
        let body = format!(r#"{{"script":"{}","args":{}}}"#,script,args);
        let resp = send_and_read_body(Method::POST, &self.execute_sync_url, cont_length_header(&body), &body);
        if resp.contains("error"){
            return Err(resp);
        }
        Ok(resp)
    }
    ///Executes the async fun in the browser. The args should be passed similarly to the execute_sync fn.
    pub fn execute_async(&self, script: &str, args: &Vec<&str>)->Result<String,String>{
        let args = gen_script_args(args);
        let body = format!(r#"{{"script":"{}","args":{}}}"#,script,args);
        let resp = send_and_read_body(Method::POST, &self.execute_async_url, cont_length_header(&body), &body);
        if resp.contains("error"){
            return Err(resp);
        }
        Ok(resp)
    }
    ///Prints out the page. If you want to print it to pdf, use headless mode. The structs PrintSettings,Page and Margin allow you to customize the print.
    pub fn print(&self,print_settings:&PrintSettings,path:&str)->Result<(),String>{
        let pr_set_body = serde_json::to_string(&print_settings).unwrap();
        let resp = send_request_screensh(Method::POST, &self.print_page_url, cont_length_header(&pr_set_body), &pr_set_body).unwrap();
        let new = base64::decode(resp).unwrap();
        std::fs::write(path,new).unwrap();
        Ok(())
    }
    
}

impl Browser{
    pub fn dismiss_alert(&self)->Result<(),String>{
        let resp = send_and_read_body(Method::POST, &self.alert_dismiss_url, cont_length_header("{}"), "{}");
        if resp ==r#"{"value":null}"#{Ok(())}else{Err(resp)}
    }
    pub fn allow_alert(&self)->Result<(),String>{
        let resp = send_and_read_body(Method::POST, &self.alert_accept_url, cont_length_header("{}"), "{}");
        if resp ==r#"{"value":null}"#{Ok(())}else{Err(resp)}
    }
    pub fn get_alert_text(&self)->Result<String,String>{
        let resp = send_and_read_body(Method::GET, &self.alert_text_url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map:HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).to_string())
    }
    pub fn send_alert_text(&self,text:&str)->Result<(),String>{
        let body =format!(r#"{{"text":{}}}"#,text) ;
        let resp = send_and_read_body(Method::POST, &self.alert_dismiss_url, cont_length_header(&body), &body);
        if resp.contains("error"){return Err(resp);}
        Ok(())
    }
}
pub (self) mod utils{
    use super::*;

    pub (super) fn create_session_body_json(browser:&str,os:&str, args:Vec<&str>)->String{
            match browser{
                "chrome"=> create_chrome_session(os,args),
                _=>panic!("Sorry, so far only chrome is supported")
            }
    }
    pub (super) fn create_chrome_session(os:&str,args:Vec<&str>)->String{
            let one=format!(r#"{{"capabilities": {{"alwaysMatch":{{"platformName":"{}"}}"#,os);
            let args = gen_args(args);
            let two=format!(r#"{},"firstMatch":[{{"browserName":"chrome","goog:chromeOptions":{{"args":{}}}}}]}}}}"#,one,args);
            two
    }

    pub (super) fn gen_args(args:Vec<&str>)->String{
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
    pub (super) fn gen_script_args(args:&Vec<&str>)->String{
        if args.len()==0{
            return String::from("[]");
        }
        let mut result = String::from("[");
        let temp_result = args.join(",");
        result.push_str(&temp_result);
        result.push_str("]");
        result
    }
    pub (super) fn from_value_to_cookie(val: &serde_json::Value)->Cookie{
        let name = String::from(val["name"].as_str().unwrap());
        let value = String::from(val["value"].as_str().unwrap());
        let mut domain = String::from("");
        let mut expiry = 0;
        let mut http_only = false;
        let mut path = String::from("");
        let mut secure = false;
        let mut same_site = String::from("");
        if let Some(dom) = val["domain"].as_str(){
            domain=String::from(dom);
        }
        if let Some(exp) = val["expiry"].as_u64(){
            expiry=exp;
        }
        if let Some(http) = val["httpOnly"].as_bool(){
            http_only=http;
        }
        if let Some(pat) = val["path"].as_str(){
            path=String::from(pat);
        }
        if let Some(sec) = val["secure"].as_bool(){
            secure = sec;
        }
        if let Some(same) = val["sameSite"].as_str(){
            same_site=String::from(same);
        }
        Cookie{name,value,path,expiry,secure,domain,httpOnly: http_only,sameSite:same_site}
    }
}
    
/*
TODO
pub struct ChromeOptions{}
*/

pub enum NewWindowType{
    Tab,
    Window
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

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Cookie{
    pub(self)domain:String,
    pub(self)expiry: u64,
    pub(self)httpOnly:bool,
    pub(self)name:String,
    pub(self)path:String,
    pub(self)secure:bool,
    pub(self)value:String,
    pub(self)sameSite:String,
}

impl Cookie{
    pub fn new_all(domain:String, expiry:u64,same_site:String, http_only:bool,name:String,path:String,secure:bool,value:String)->Self{
        Cookie{
            domain,
            expiry,
            httpOnly: http_only,
            name,
            path,
            sameSite:same_site,
            secure,
            value
        }
    }
    pub fn new(name:String,value:String)->Self{
        Cookie{name,value,..Default::default()}
    }

    pub fn get_domain(&self)->String{self.domain.clone()}
    pub fn get_expiry(&self)->u64{self.expiry}
    pub fn get_http_only(&self)->bool{self.httpOnly}
    pub fn get_name(&self)->String{self.name.clone()}
    pub fn get_path(&self)->String{self.path.clone()}
    pub fn get_secure(&self)->bool{self.secure}
    pub fn get_value(&self)->String{self.value.clone()}
    pub fn get_same_site(&self)->String{self.sameSite.clone()}


    pub fn set_domain(&mut self, domain:String){self.domain=domain;}
    pub fn set_expiry(&mut self, expiry: u64){self.expiry=expiry;}
    pub fn set_http_only(&mut self, http_only:bool){self.httpOnly=http_only;}
    pub fn set_path(&mut self, path: String){self.path=path;}
    pub fn set_secure(&mut self, secure: bool){self.secure=secure;}
    pub fn set_same_site(&mut self, same_site:String){self.sameSite=same_site;}
}
impl Default for Cookie{
    fn default()->Self{
        Cookie{
            domain:"".to_string(),
            expiry: 0,
            httpOnly: false,
            name:"".to_string(),
            path:"".to_string(),
            secure:false,
            value:"".to_string(),
            sameSite:"None".to_string(),
        }
    }
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
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug)]
pub struct PrintSettings{
    orientation:String,
    scale: f32,
    background: bool,
    page: Page,
    margin: Margin,
    shrinkToFit: bool,
    pages: Vec<u32>
}
impl PrintSettings{
    pub fn new(orientation: Orientation,scale: f32,background:bool,page:Page,margin:Margin,shrink_tf:bool,pages:Vec<u32>)->Self{
        if scale<0.1||scale>2.0{panic!("The condotion (0.1<=scale<= 2.0) is not fulfilled");}
        let orientation = match orientation{
            Orientation::PORTRAIT=>String::from("portrait"),
            Orientation::LANDSCAPE=>String::from("landscape"),
        };
        PrintSettings{orientation,scale,background,page,margin,shrinkToFit: shrink_tf,pages}
    }
    pub fn set_orientation(&mut self,orientation: Orientation){
        self.orientation = match orientation{
            Orientation::PORTRAIT=>String::from("portrait"),
            Orientation::LANDSCAPE=>String::from("landscape"),
        };
    }
    pub fn set_scale(&mut self,scale:f32){
        if scale<0.1||scale>2.0{panic!("The condotion (0.1<=scale<= 2.0) is not fulfilled");}
        self.scale=scale;}
    pub fn set_background(&mut self){}
    pub fn set_page(&mut self,page:Page){self.page=page;}
    pub fn set_margin(&mut self,margin:Margin){self.margin=margin;}
    pub fn set_shrink_to_fit(&mut self,shr_to_fit:bool){self.shrinkToFit = shr_to_fit;}
    pub fn set_pages(&mut self,pages:Vec<u32>){self.pages=pages;}
}
/*TODO !!!
Методы для создания и настройки PrintSettings, добавить валидации на поля*/
impl Default for PrintSettings{
    fn default()->Self{
        PrintSettings{
            orientation:String::from("portrait"),
            scale: 1.0,
            background: false,
            page: Page::default(),
            margin: Margin::default(),
            shrinkToFit: true,
            pages: vec![] 
        }
    }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Page{
    width:f32,
    height:f32,
}
impl Page{
    pub fn new(width:f32, height: f32)->Self{
        if width<0.0||height<0.0{panic!("Width and height can't be less then 0.0");}
        Page{width,height}
    }
    pub fn set_width(&mut self,width:f32){
        if width<0.0{panic!("Width cannot be less then 0.0");}
        self.width = width;
    }
    pub fn set_height(&mut self,height:f32){
        if height<0.0{panic!("Height cannot be less then 0.0");}
        self.height = height;
    }
}
impl Default for Page{
    fn default()->Self{
        Page{width:21.59,height:27.94}
    }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Margin{
    top:u32,
    bottom:u32,
    left:u32,
    right:u32,
}
impl Margin{
    pub fn new(top:u32,bottom:u32,left:u32,right:u32)->Self{
        Margin{
            top,bottom,left,right
        }
    }
    pub fn set_top(&mut self,top:u32){self.top=top;}
    pub fn set_bottom(&mut self,bottom:u32){self.bottom=bottom}
    pub fn set_left(&mut self,left:u32){self.left=left;}
    pub fn set_right(&mut self,right:u32){self.right=right;}
}
impl Default for Margin{
    fn default()->Self{
        Margin{
            top:1,
            bottom:1,
            left:1,
            right:1,
        }
    }
}
pub enum Orientation{
    PORTRAIT,
    LANDSCAPE
}

//TESTS
pub mod tests{

    use super::*;
    use super::Element;
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
        assert_eq!(&link,"https://vk.com/");
    }
    #[test]
    #[should_panic]
    fn close_browser() {
        let mut browser = Browser::start_session("chrome", consts::OS,vec!["--headless"]);
        browser.open("http://localhost:4444/wd/hub/status");
        browser.close_browser();        
        let a = browser.get_link();
        if a.contains("invalid"){panic!("Invalid session id");}
    }
    #[test]
    fn args() {
        let a = vec!["--headless","--window-size=800,400"];        
        assert!(gen_args(a)==String::from("[\"--headless\",\"--window-size=800,400\"]"));
    }
    #[test]
    fn get_timeouts() {
        let timeouts:Timeouts;
        {
            let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
            timeouts= br.get_timeouts();
            br.close_browser();
        }
        assert!(timeouts.implicit==0&&timeouts.script==30000);
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
        assert_eq!(&link,"https://m.facebook.com/");
    }
    #[test]
    fn refresh_test() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com/");
        assert_eq!(br.refresh(),Ok(()));
        br.close_browser();
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
        let len = handles.len();
        assert_eq!(len,1);
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
        let el = br.find_element(LocatorStrategy::CSS("#quick_login_frame"));
        let res = br.switch_to_frame_by_element(el);
        br.close_browser();
        assert_eq!(res,Ok(()));
    }
    #[test]
    fn find_element() {
        let el;
        {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        el = br.find_element(LocatorStrategy::CSS("#ts_input"));
        br.close_browser();
        }
        let tr =el.element_gr_id.contains("element"); 
        assert!(tr);
    }
    #[test]
    fn find_els() {
        let el;
        {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://bash.im");
        el = br.find_elements(LocatorStrategy::CSS("article"));
        br.close_browser();
        }
        let len = el.len();
        assert!(len>2);
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
    #[test]
    fn parse_val_test() {
        let resp = r#"{
       "value": {
       "dftg43rert34tert-34trte-243f-4":
       {
        "id": 333
        }
      }
     }"#;
       let res = parse_value(resp);
       assert_eq!(res,"{\"dftg43rert34tert-34trte-243f-4\":{\"id\":333}}");
    }
    #[test]
    fn fullsize() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        let a = br.fullscreen().unwrap();
        br.close_browser();
        assert!(a.x==0&&a.y==0);
    }

    #[test]
    fn src() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        br.open("http://localhost:4444/wd/hub/status");
        let a = br.source();
        br.close_browser();
        assert!(a.contains("html")&&a.contains("head"))
    }

    #[test]
    fn script_test() {
        let script = "return 3+2";
        let res = vec!["5",r#""Hello""#];
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        let res = br.execute_sync(script, &res).unwrap();
        br.close_browser();
        assert!(res.contains("5"));
    }
    
    #[test]
    fn cookies() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let cook = br.get_all_cookies();
        br.close_browser();
        assert!(cook.len()>1);
    }

    #[test]
    fn get_cookie() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let c = br.get_cookie("tmr_lvidTS").unwrap();
        br.close_browser();
        assert_eq!(c.httpOnly,false);
    }

    #[test]
    fn add_cookie() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let cook = Cookie::new_all(String::from(""), 1000, String::from("Lax"), false, String::from("tmr_detect"), String::from(""), false, String::from("0%7C1604223632022"));
        assert_eq!(br.add_cookie(cook),Ok(()));
        br.close_browser();
    }

    #[test]
    fn a_del_all_cook() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        //std::thread::sleep_ms(200);
        br.delete_all_cookies().unwrap();
        let cook = br.get_all_cookies();
        //dbg!(&cook);
        br.close_browser();
        assert_eq!(cook.len(),0);
    }
    #[test]
    fn del_cookie() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let r = br.delete_cookie("remixjsp");
        br.close_browser();
        assert!(r==Ok(()));
    }
    #[test]
    fn screensh() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=7680,4320"]);
        br.open("https://vk.com");
        br.take_screenshot("screen.png").unwrap();
        br.close_browser();
        let arr=std::fs::read("screen.png").unwrap().len();
        assert!(arr>0);
        std::fs::remove_file("screen.png").unwrap();
    }
    #[test]
    fn el_screensh() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=800,600"]);
        br.open("https://vk.com");
        let el = br.find_element(LocatorStrategy::CSS("#ts_input"));
        br.take_element_screenshot(&el,"element.png").unwrap();
        br.close_browser();
        let arr=std::fs::read("element.png").unwrap().len();
        assert!(arr>0);
        std::fs::remove_file("element.png").unwrap();
    }
    #[test]
    fn pr_page() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=2400,1200"]);
        br.open("https://vk.com");
        let mut p  = PrintSettings::default();
        p.set_orientation(Orientation::LANDSCAPE);
        p.set_pages(vec![1,2]);
        br.print(&p, "page.pdf").unwrap();
        br.close_browser();
        let arr=std::fs::read("page.pdf").unwrap().len();
        assert!(arr>0);
        std::fs::remove_file("page.pdf").unwrap();
    }

    #[test]
    fn alerts() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let resp = br.dismiss_alert().is_err();
        let resp2 = br.allow_alert().is_err();
        br.close_browser();
        assert!(resp&&resp2);
    }
    #[test]
    fn get_send_alert_text() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let resp = br.get_alert_text().is_err();
        let resp2 = br.send_alert_text("I am the text").is_err();
        br.close_browser();
        assert!(resp&&resp2);
    }
    #[test]
    fn active_el() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless"]);
        br.open("https://vk.com");
        let a = br.get_active_element().unwrap();
        br.close_browser();
        assert!(a.element_gr_id.contains("element"));
    }
    #[test]
    fn find_sub_el() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        br.open("https://vk.com");
        let par_el= br.find_element(LocatorStrategy::CSS("#top_nav"));
        let a = par_el.find_element_from_self(LocatorStrategy::CSS(".HeaderNav__item")).unwrap();
        br.close_browser();
        assert!(a.element_gr_id.contains("element"));
    }
    #[test]
    fn sub_els() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        br.open("https://bash.im");
        let par_el= br.find_element(LocatorStrategy::CSS("section.quotes"));
        let a = par_el.find_elements_from_self(LocatorStrategy::CSS(".quote")).unwrap();
        br.close_browser();
        let len  = a.len();
        assert!(len>2);
    }
    #[test]
    fn is_sel() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        br.open("https://vk.com");
        let el= br.find_element(LocatorStrategy::CSS("#ts_input"));
        let res = el.is_selected().unwrap();
        br.close_browser();
        assert!(res==false);
    }
    #[test]
    fn get_arrtib() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        br.open("https://vk.com");
        let el= br.find_element(LocatorStrategy::CSS(".placeholder_content"));
        let res = el.get_attribute("aria-hidde").unwrap();
        let res2 = el.get_attribute("aria-hidden").unwrap();
        br.close_browser();
        assert!(res.as_str()=="null");
        assert!(res2.as_str()=="true");
    }
    #[test]
    fn get_property() {
        let mut br = Browser::start_session("chrome", consts::OS, vec!["--headless","--window-size=400,200"]);
        br.open("https://vk.com");
        let el= br.find_element(LocatorStrategy::CSS("#index_login_form"));
        let res_len = el.get_property("attributes").unwrap().len();
        let res_null = el.get_property("attributes2").unwrap();
        br.close_browser();
        assert!(res_len>5000&&res_null.as_str()=="null");
    }
}