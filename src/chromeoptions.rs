///Utility struct to adjust the chrome browser session
/// 
/// For more info pls check the chromedriver docs https://chromedriver.chromium.org/capabilities
pub struct ChromeOptions{
    pub(crate) string_for_session:String,
}
impl ChromeOptions{
    pub fn new()->Self{
        ChromeOptions{
            string_for_session: String::from(r#""goog:chromeOptions":{}"#),
        }
    }
    pub fn add_args(&mut self,args:Vec<&str>)->&mut Self{
        if self.string_for_session.contains("args"){panic!("The options already contain args");}
        self.string_for_session.pop();
        let mut inner_args = String::from("\"args\":[");
        for st in args{
            inner_args.push('"');
            inner_args.push_str(st);
            inner_args.push('"');
            inner_args.push(',');
        }
        inner_args.pop();
        inner_args.push(']');
        inner_args.push(',');
        self.string_for_session.push_str(&inner_args);
        self.string_for_session.push('}');
        self
    }
    pub fn add_binary(&mut self,path: &str)->&mut Self{
        if self.string_for_session.contains("binary"){panic!("The options already contain path to binary");}
        self.string_for_session.pop();
        let bin = format!(r#""binary":"{}","#,path);
        self.string_for_session.push_str(&bin);
        self.string_for_session.push('}');
        self
    }
    //Each item in the vec should be a base-64 encoded packed Chrome extension (.crx)
    pub fn add_extensions(&mut self,extensions:Vec<&str>)->&mut Self{
        if self.string_for_session.contains("extensions"){panic!("The options already contain extension");}
        self.string_for_session.pop();
        let mut inner_args = String::from("\"extensions\":[");
        for st in extensions{
            inner_args.push('"');
            inner_args.push_str(st);
            inner_args.push('"');
            inner_args.push(',');
        }
        inner_args.pop();
        inner_args.push(']');
        inner_args.push(',');
        self.string_for_session.push_str(&inner_args);
        self.string_for_session.push('}');
        self
    }
    ///A string representation of the json with preferences names and values.
    pub fn add_local_state(&mut self,local_state: &str)->&mut Self{
        if self.string_for_session.contains("localState"){panic!("The options already contain local state");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    ///A string representation of the json with preferences names and values.
    pub fn add_prefs(&mut self,prefs: &str)->&mut Self{
        if self.string_for_session.contains("prefs"){panic!("The options already contain prefs");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    pub fn add_detach(&mut self,detach: bool)->&mut Self{
        if self.string_for_session.contains("detach"){panic!("The options already contain detach");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    pub fn add_debugger_address(&mut self,address: &str)->&mut Self{
        if self.string_for_session.contains("debuggerAddress"){panic!("The options already contain debugger address");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    pub fn add_exclude_switches(&mut self,switches: Vec<&str>)->&mut Self{
        if self.string_for_session.contains("excludeSwitches"){panic!("The options already contain switches to exclude");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    pub fn add_minidump_path(&mut self,path:&str)->&mut Self{
        if self.string_for_session.contains("minidumpPath"){panic!("The options already contain the path to the minidump");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    pub fn add_mobile_emulation(&mut self,device:MobileDevice)->&mut Self{
        if self.string_for_session.contains("mobileEmulation"){panic!("The options already contain the device to emulate");}
        self.string_for_session.pop();
        self.string_for_session.push('}');
        self
    }
    pub fn add_window_types(&mut self,window_types:Vec<&str>)->&mut Self{
        if self.string_for_session.contains("windowTypes"){panic!("The options already contain window types");}
        self.string_for_session.pop();
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
pub mod chr_opt_tests{
    use super::*;
    #[test]
    fn gen_str() {
        let v = vec![];
        let string = from_str_vec_to_str(v);
        assert_eq!(&string, "[]");
        let args = vec!["--headless","--window-size=800,600"];
        let string = from_str_vec_to_str(args);
        assert_eq!(string,"[\"--headless\",\"--window-size=800,600\"]");
    }
}