use std::collections::HashMap;
use serde::Serialize;
///Utility struct to adjust the chrome browser session
/// 
/// For more info pls check the chromedriver docs at https://chromedriver.chromium.org/capabilities
pub struct ChromeOptions{
    pub(crate) string_for_session:String,
}
impl ChromeOptions{
    pub fn new()->Self{
        ChromeOptions{
            string_for_session: String::from(r#""goog:chromeOptions":{}"#),
        }
    }
    ///List of arguments to be passed to the Chrome browser on launch
    /// # Examples
    /// ```
    /// # use selenium_webdriver::*;
    /// let args = vec!["--headless","--window-size=800,600"];
    /// let mut ch = ChromeOptions::new();
    /// ch.add_args(args);
    /// ```
    pub fn add_args(&mut self,args:Vec<&str>)->&mut Self{
        if self.string_for_session.contains("args"){panic!("The options already contain args");}
        self.string_for_session.pop();
        let mut inner_args = String::from("\"args\":");
        let vec_to_str = from_str_vec_to_str(args);
        inner_args.push_str(&vec_to_str);
        self.string_for_session.push_str(&inner_args);
        self.string_for_session.push('}');
        self
    }
    ///Path to chrome executable
    pub fn add_binary(&mut self,path: &str)->&mut Self{
        if self.string_for_session.contains("binary"){panic!("The options already contain path to binary");}
        self.string_for_session.pop();
        let bin = format!(r#""binary":"{}","#,path);
        self.string_for_session.push_str(&bin);
        self.string_for_session.push('}');
        self
    }
    ///Each item in the vec should be a base-64 encoded packed Chrome extension (.crx)
    pub fn add_extensions(&mut self,extensions:Vec<&str>)->&mut Self{
        if self.string_for_session.contains("extensions"){panic!("The options already contain extension");}
        self.string_for_session.pop();
        let mut inner_args = String::from("\"extensions\":");
        let vec_to_str = from_str_vec_to_str(extensions);
        inner_args.push_str(&vec_to_str);
        self.string_for_session.push_str(&inner_args);
        self.string_for_session.push('}');
        self
    }
    ///Local state preferences' names and values.
    pub fn add_local_state(&mut self,local_state: HashMap<&str,&str>)->&mut Self{
        if self.string_for_session.contains("localState"){panic!("The options already contain local state");}
        self.string_for_session.pop();
        let mut temp_string = String::from("\"localState\":{");
        for i in local_state{
            let temp = format!(r#""{}":"{}","#,i.0,i.1);
            temp_string.push_str(&temp);
        }
        temp_string.pop();
        temp_string.push('}');
        self.string_for_session.push('}');
        self
    }
    ///User profile preferences' names and values.
    pub fn add_prefs(&mut self,prefs: HashMap<&str,&str>)->&mut Self{
        if self.string_for_session.contains("prefs"){panic!("The options already contain prefs");}
        self.string_for_session.pop();
        let mut temp_string = String::from("\"prefs\":{");
        for i in prefs{
            let temp = format!(r#""{}":"{}","#,i.0,i.1);
            temp_string.push_str(&temp);
        }
        temp_string.pop();
        temp_string.push('}');
        self.string_for_session.push('}');
        self
    }
    ///If false, Chrome will be quit when ChromeDriver is killed even if the session is still active
    pub fn add_detach(&mut self,detach: bool)->&mut Self{
        if self.string_for_session.contains("detach"){panic!("The options already contain detach");}
        self.string_for_session.pop();
        let text = format!(r#""detach":{},"#,detach);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///An address of a Chrome debugger server to connect to, for example, e.g. "127.0.0.1:38947"
    pub fn add_debugger_address(&mut self,address: &str)->&mut Self{
        if self.string_for_session.contains("debuggerAddress"){panic!("The options already contain debugger address");}
        self.string_for_session.pop();
        let text = format!(r#""debuggerAddress":"{}","#,address);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///List of Chrome command line switches to exclude 
    pub fn add_exclude_switches(&mut self,switches: Vec<&str>)->&mut Self{
        if self.string_for_session.contains("excludeSwitches"){panic!("The options already contain switches to exclude");}
        self.string_for_session.pop();
        let mut temp_string = String::from("\"excludeSwitches\":");
        let vec_to_str = from_str_vec_to_str(switches);
        temp_string.push_str(&vec_to_str);
        self.string_for_session.push_str(&temp_string);
        self.string_for_session.push('}');
        self
    }
    ///Only for Linux. Directory to store Chrome minidumps
    pub fn add_minidump_path(&mut self,path:&str)->&mut Self{
        if self.string_for_session.contains("minidumpPath"){panic!("The options already contain the path to the minidump");}
        self.string_for_session.pop();
        let text = format!(r#""minidumpPath":"{}","#,path);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///See the MobileDevice struct for more info
    pub fn add_mobile_emulation(&mut self,device:MobileDevice)->&mut Self{
        if self.string_for_session.contains("mobileEmulation"){panic!("The options already contain the device to emulate");}
        self.string_for_session.pop();
        let text = format!(r#""mobileEmulation":{},"#,device.device_dict);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///Pls check chromedriver docs for more info
    pub fn add_perf_logging_prefs(&mut self, prefs:PerfLoggingPrefs)->&mut Self{
        if self.string_for_session.contains("perfLoggingPrefs"){panic!("The options already contain the device to emulate");}
        self.string_for_session.pop();
        let perf_to_json = serde_json::to_string(&prefs).unwrap();
        let text = format!(r#""perfLoggingPrefs":{},"#,perf_to_json);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///A list of window types that will appear in the list of window handles.
    pub fn add_window_types(&mut self,window_types:Vec<&str>)->&mut Self{
        if self.string_for_session.contains("windowTypes"){panic!("The options already contain window types");}
        self.string_for_session.pop();
        let mut temp_string = String::from("\"windowTypes\":");
        let vec_to_str = from_str_vec_to_str(window_types);
        temp_string.push_str(&vec_to_str);
        self.string_for_session.push_str(&temp_string);
        self.string_for_session.push('}');
        self
    }
}
fn from_str_vec_to_str(vec: Vec<&str>)->String{
    if vec.len() == 0 {return String::from("[]");}
    let mut string_args = String::from("[");
    for st in vec{
        string_args.push('"');
        string_args.push_str(st);
        string_args.push('"');
        string_args.push(',');
    }
    string_args.pop();
    string_args.push(']');
    string_args.push(',');
    string_args
}
///See the details at https://chromedriver.chromium.org/mobile-emulation
pub struct MobileDevice{
    pub(crate) device_dict:String,
}
impl MobileDevice{
    ///Method to select to standard device from the Chrome devtools
    pub fn standard_device(device_name:&str)->Self{
        let device_dict = format!(r#"{{"deviceName":"{}"}}"#,device_name);
        MobileDevice{
            device_dict
        }
    }
    ///Method to the create a custom mobile device to emulate
    pub fn custom_device(width:u32,
                        height:u32,
                        pixel_ratio:f32,
                        touch: bool,
                        user_agent: &str)->Self{
        let device_dict = format!(
            r#"{{"deviceMetrics":{{
            "width":{},
            "height":{},
            "pixel_ratio":{},
            "touch":{}}},
        "userAgent":"{}"}}"#,width,height,pixel_ratio,touch,user_agent).
        replace("\n","").
        replace(" ","");
        MobileDevice{device_dict}
    }
}
///An optional struct that specifies performance logging preferences
/// 
/// Has the default method for instantiating and setters for customization
#[allow(non_snake_case)]
#[derive(Serialize,Debug)]
pub struct PerfLoggingPrefs{
    enableNetwork:bool,
    enablePage:bool,
    traceCategories:String,
    bufferUsageReportingInterval:u32,
}
impl PerfLoggingPrefs{
    pub fn set_enable_network(&mut self, en_network:bool){
        self.enableNetwork = en_network;
    }
    pub fn set_enable_page(&mut self,en_page:bool ){
        self.enablePage = en_page;
    }
    pub fn set_trace_categories(&mut self,trace_cat:String){
        self.traceCategories = trace_cat;
    }
    pub fn set_buffer_usage_rep_interval(&mut self,buff: u32){
        self.bufferUsageReportingInterval = buff;
    }     
}
impl Default for PerfLoggingPrefs{
    fn default()->Self{
        PerfLoggingPrefs{
            enableNetwork: true,
            enablePage: true,
            traceCategories: String::from(""),
            bufferUsageReportingInterval: 1000,
        }
    }
}



pub (crate) mod chr_opt_tests{
    use super::*;
    #[test]
    fn chro_mob_em() {
        let mut ch = ChromeOptions::new();
        let mob = MobileDevice::standard_device("Nexus 6");
        ch.add_mobile_emulation(mob);
        let x = r#""goog:chromeOptions":{"mobileEmulation":{"deviceName":"Nexus 6"},}"#;
        assert_eq!(x,ch.string_for_session);
    }
    #[test]
    fn chro_cust_mob_em() {
        let mut ch = ChromeOptions::new();
        let mob = MobileDevice::custom_device(300,150,3.0,true,"Custom Agent");
        ch.add_mobile_emulation(mob);
        let x = r#""goog:chromeOptions":{"mobileEmulation":{"deviceMetrics":{"width":300,"height":150,"pixel_ratio":3,"touch":true},"userAgent":"CustomAgent"},}"#;
        assert_eq!(x,ch.string_for_session);
    }
    #[test]
    fn chro_gen_str() {
        let v = vec![];
        let string = from_str_vec_to_str(v);
        assert_eq!(&string, "[]");
        let args = vec!["--headless","--window-size=800,600"];
        let string = from_str_vec_to_str(args);
        assert_eq!(string,"[\"--headless\",\"--window-size=800,600\"],");
    }
    #[test]
    fn chro_gen_str_for_ext() {
        let v = vec!["ext_one","ext_two"];
        let mut ch = ChromeOptions::new();
        ch.add_extensions(v);
        assert_eq!(r#""goog:chromeOptions":{"extensions":["ext_one","ext_two"],}"#,ch.string_for_session);
    }
    #[test]
    fn chro_gen_str_for_args() {
        let args = vec!["--headless","--window-size=800,600"];
        let mut ch = ChromeOptions::new();
        ch.add_args(args);
        assert_eq!(r#""goog:chromeOptions":{"args":["--headless","--window-size=800,600"],}"#,ch.string_for_session);
    }
    #[test]
    fn chro_gen_str_for_window_types() {
        let types = vec!["type_one","type_two"];
        let mut ch = ChromeOptions::new();
        ch.add_window_types(types);
        assert_eq!(r#""goog:chromeOptions":{"windowTypes":["type_one","type_two"],}"#,ch.string_for_session);
        
    }
    #[test]
    fn chro_perf_log() {
        let pr = PerfLoggingPrefs::default();
        let mut ch = ChromeOptions::new();
        ch.add_perf_logging_prefs(pr);
        assert!(ch.string_for_session.contains("perfLoggingPrefs"));
    }
}
