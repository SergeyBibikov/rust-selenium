use super::{BrowserName,Timeouts};
use super::ChromeOptions;
use super::FirefoxOptions;
use super::SafariOptions;
use super::{Proxy,ProxyType};
///Utility struct to set up the session
/// 
/// The capabilities will always match, meaning that even if a single requirement cannot
/// be met, the session initialization will fail. The new() constructor takes two
/// arguments as it is impossible to start a session without the browser and platform info.
/// Other capabilities are optional.
/// 
/// For more info please visit https://www.w3.org/TR/webdriver/#capabilities
pub struct Capabilities{
   pub (crate) cap_string: String,
}

impl Capabilities{

    pub fn new(browser: BrowserName,platform: &str)->Self{
        let browser = match browser{
            BrowserName::Chrome=>"chrome",
            BrowserName::Firefox=>"firefox",
            BrowserName::Safari=>"safari",
        };
        let cap_string = format!(r#"
        {{
            "capabilities": {{
                "alwaysMatch": {{
                    "browserName": "{}",
                    "platformName": "{}"}}}}}}"#,browser,platform);
        Capabilities{cap_string}
    }
    pub fn set_chrome_options(&mut self, options:ChromeOptions)->&mut Self{
        let text = options.string_for_session;
        update(&mut self.cap_string,&text);
        for _ in 0..5{self.cap_string.pop();}
        for _ in 0..4{self.cap_string.push('}');}
        self
    }
    pub fn set_firefox_options(&mut self, options:FirefoxOptions)->&mut Self{
        let text = options.string_for_session;
        update(&mut self.cap_string,&text);
        for _ in 0..5{self.cap_string.pop();}
        for _ in 0..4{self.cap_string.push('}');}
        self
    }
    pub fn set_safari_options(&mut self, options:SafariOptions)->&mut Self{
        self
    }
    pub fn set_browser_version(&mut self, version: &str)->&mut Self{
        let text = format!(r#""browserVersion":"{}""#,version);
        update(&mut self.cap_string,&text);
        self
    }
    pub fn enable_insecure_certs(&mut self)->&mut Self{
        let text = r#""acceptInsecureCerts":true"#;
        update(&mut self.cap_string,text);
        self
    }
    pub fn set_pageload_strategy(&mut self,strategy: &str)->&mut Self{
        let text = format!(r#""pageLoadStrategy":"{}""#,strategy);
        update(&mut self.cap_string,&text);
        self
    }
    pub fn set_proxy(&mut self, proxy: Proxy)->&mut Self{
        let text = format!(r#""proxy":{}"#,proxy.proxy_string);
        update(&mut self.cap_string,&text);
        self
    }
    pub fn disable_window_rect(&mut self)->&mut Self{
        let text = r#""setWindowRect":false"#;
        update(&mut self.cap_string,text);
        self
    }
    pub fn set_timeouts(&mut self,timeouts: Timeouts)->&mut Self{
        let text = format!(r#""timeouts":{}"#,serde_json::to_string(&timeouts).unwrap());
        update(&mut self.cap_string,&text);
        self
    }
    pub fn enable_strict_file_interact(&mut self)->&mut Self{
        let text = r#""strictFileInteractability":true"#;
        update(&mut self.cap_string,text);
        self
    }
    pub fn set_unhandled_prompt_behavior(&mut self, behavior:&str)->&mut Self{
        let text = format!(r#""unhandledPromptBehavior":"{}""#,behavior);
        update(&mut self.cap_string,&text);
        self
    }
    
}
fn update(st:&mut String,text:&str){
    st.pop();
    st.pop();
    st.pop();
    st.push(',');
    st.push_str(text);
    st.push('}');
    st.push('}');
    st.push('}');
}

mod capab_tests{
    use super::*;
    use super::super::LogLevel;
    #[test]
    fn cap_chr_ops() {
        let mut ch_op = ChromeOptions::new();
        ch_op.add_args(vec!["--headless","--window-size=800,600"]);
        ch_op.add_binary("C:\\User\\Me\\bin");
        ch_op.add_debugger_address("127.0.0.1:8990");
        let mut c = Capabilities::new(BrowserName::Chrome, "linux");
        c.set_chrome_options(ch_op);
        let x = r#"{"capabilities":{"alwaysMatch":{"browserName": "chrome", "platformName": "linux","goog:chromeOptions":{"args":["--headless","--window-size=800,600"],"binary":"C:\User\Me\bin","debuggerAddress":"127.0.0.1:8990"}}}}"#;
        let x = x.replace(" ","");
        let res_st = c.cap_string.replace("\n","").replace(" ","");
        assert_eq!(x,res_st);
    }
    #[test]
    fn cap_fire_ops() {
        let mut ff_op = FirefoxOptions::new();
        ff_op.add_log(LogLevel::Info).add_binary("C:\\User\\Me\\bin")
        .add_prefs(r#"{"one pref":"one pref val"}"#);
        let mut c = Capabilities::new(BrowserName::Firefox, "windows");
        c.set_firefox_options(ff_op);
        let x = r#"{"capabilities": {"alwaysMatch": {"browserName": "firefox","platformName": "windows","moz:firefoxOptions":{"log":{"level":"info"},"binary":"C:\User\Me\bin","prefs":{"one pref":"one pref val"}}}}}"#;
        let x = x.replace(" ","");
        let res_st = c.cap_string.replace("\n","").replace(" ","");
        assert_eq!(x,res_st);
    }
    #[test]
    fn cap_all() {
        let mut cap = Capabilities::new(BrowserName::Firefox, "windows");
        let t = Timeouts::new();
        let mut prox = Proxy::new();
        prox.set_proxy_type(ProxyType::Pac)
        .set_proxy_autoconfig_url("http://my.com")
        .set_socks_version(220)
        .set_no_proxy(vec!["one","two"])
        .set_ftp_proxy("host:port")
        .set_http_proxy("host:port")
        .set_ssl_proxy("host:port")
        .set_socks_proxy("host:port");
        cap
        .enable_insecure_certs()
        .disable_window_rect()
        .enable_strict_file_interact()
        .set_timeouts(t)
        .set_proxy(prox)
        .set_unhandled_prompt_behavior("do something!!")
        .set_pageload_strategy("the best strategy")
        .set_browser_version("86.0.0.1");
        let len = cap.cap_string.len();
        let last = &cap.cap_string[len-4..];
        //println!("{}",last.contains(","));
        assert!(!last.contains(","));
    }
}