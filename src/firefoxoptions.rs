use super::reqs::*;
use std::collections::HashMap;
/// Utility struct to adjust the firefox browser session
/// 
/// For more info on FF Options please check
/// https://developer.mozilla.org/en-US/docs/Web/WebDriver/Capabilities/firefoxOptions
pub struct FirefoxOptions{
    pub(crate) string_for_session:String,
}
impl FirefoxOptions{
    pub fn new()->Self{
        FirefoxOptions{
            string_for_session: String::from(r#""moz:firefoxOptions":{}"#),
        }
    }
    ///Absolute path to the custom Firefox binary to use.
    pub fn add_binary(&mut self,path: &str)->&mut Self{
        if self.string_for_session.contains("binary"){panic!("The options already contain path to binary");}
        self.string_for_session.pop();
        let bin = format!(r#""binary":"{}","#,path);
        self.string_for_session.push_str(&bin);
        self.string_for_session.push('}');
        self
    }
    ///More info on the FF args here:
    ///https://developer.mozilla.org/en-US/docs/Mozilla/Command_Line_Options?redirectlocale=en-US&redirectslug=Command_Line_Options
    pub fn add_args(&mut self, args: Vec<&str>)->&mut Self{
        if self.string_for_session.contains("args"){panic!("The options already contain args");}
        self.string_for_session.pop();
        let mut inner_args = String::from("\"args\":");
        let vec_to_str = from_str_vec_to_str(args);
        inner_args.push_str(&vec_to_str);
        self.string_for_session.push_str(&inner_args);
        self.string_for_session.push('}');
        self
    }
    ///As there are various prefs types, this methods takes a string representation of json.
    /// # Examples
    /// ```
    /// # use selenium_webdriver::*;
    /// let mut ff = FirefoxOptions::new();
    /// let prefs = r#"{dom.ipc.processCount": 8,"javascript.options.showInConsole": false}"#;
    /// ff.add_prefs(prefs);
    /// ```
    pub fn add_prefs(&mut self, prefs:&str)->&mut Self{
        if self.string_for_session.contains("prefs"){panic!("The options already contain prefs");}
        self.string_for_session.pop();
        let text = format!(r#""prefs":{},"#,prefs);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///Option to increase the logging verbosity of geckodriver
    pub fn add_log(&mut self, log_level:LogLevel)->&mut Self{
        if self.string_for_session.contains("log"){panic!("The options already contain log");}
        self.string_for_session.pop();
        let lev_to_str = match log_level{
            LogLevel::Trace=>"trace",
            LogLevel::Debug=>"debug",
            LogLevel::Config=>"config",
            LogLevel::Info=>"info",
            LogLevel::Warn=>"warn",
            LogLevel::Error=>"error",
            LogLevel::Fatal=>"fatal"
        };
        let text = format!(r#""log":{{"level":"{}"}},"#,lev_to_str);
        self.string_for_session.push_str(&text);
        self.string_for_session.push('}');
        self
    }
    ///Map of environment variable name to environment variable value
    pub fn add_env(&mut self,env_vars: HashMap<&str,&str>)->&mut Self{
        if self.string_for_session.contains("env"){panic!("The options already contain env vars");}
        self.string_for_session.pop();
        let mut temp_string = String::from("\"env\":{");
        for i in env_vars{
            let temp = format!(r#""{}":"{}","#,i.0,i.1);
            temp_string.push_str(&temp);
        }
        temp_string.pop();
        temp_string.push('}');
        temp_string.push(',');
        self.string_for_session.push_str(&temp_string);
        self.string_for_session.push('}');
        self
    }
}
pub enum LogLevel{
    Trace,
    Debug,
    Config,
    Info,
    Warn,
    Error,
    Fatal
}

mod firefox_tests{
    use super::*;
    #[test]
    fn firef_opts() {
        let map = r#"{"one":2,"two":"two_val"}"#;
        let mut ff = FirefoxOptions::new();
        ff.add_binary("C\\Users\\Me");
        ff.add_prefs(map);
        ff.add_args(vec!["-headless","-devtools"]);
        ff.add_log(LogLevel::Error);
        let mut m = HashMap::new();
        m.insert("first var","first var val");
        ff.add_env(m);
        let x = r#""moz:firefoxOptions":{"binary":"C\Users\Me","prefs":{"one":2,"two":"two_val"},"args":["-headless","-devtools"],"log":{"level":"error"},"env":{"first var":"first var val"},}"#;
        assert_eq!(x,ff.string_for_session);
    }
}