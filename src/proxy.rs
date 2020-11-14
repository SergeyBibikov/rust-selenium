///Utility struct for session capabilities defining the proxy settings
/// 
/// For more details please see https://www.w3.org/TR/webdriver/#proxy
pub struct Proxy{
    proxy_string: String
}
impl Proxy{
    pub fn new()->Self{
        Proxy{
            proxy_string: String::from("{"),
        }
    }
    ///Indicates the type of proxy configuration.
    pub fn set_proxy_type(&mut self, proxy_type: ProxyType)->&mut Self{
        let text = match proxy_type{
            ProxyType::Pac=>r#""proxyType":"pac"}"#,
            ProxyType::System=>r#""proxyType":"system"}"#,
            ProxyType::Manual=>r#""proxyType":"manual"}"#,
            ProxyType::Direct=>r#""proxyType":"direct"}"#,
            ProxyType::Autodetect=>r#""proxyType":"autodetect"}"#
        };
        construct_the_str(&mut self.proxy_string,text);
        self
    }
    ///Defines the URL for a proxy auto-config file if proxyType is equal to "pac".
    pub fn set_proxy_autoconfig_url(&mut self,url: &str)->&mut Self{
        let text =format!(r#""proxyAutoconfigUrl":"{}"}}"#,url);
        construct_the_str(&mut self.proxy_string, &text);
        self
    }
    ///Defines the proxy host for FTP traffic when the proxyType is "manual".
    pub fn set_ftp_proxy(&mut self,host_and_port:&str)->&mut Self{
        let text =format!(r#""ftpProxy":"{}"}}"#,host_and_port);
        construct_the_str(&mut self.proxy_string,&text);
        self
    }
    ///Defines the proxy host for HTTP traffic when the proxyType is "manual".
    pub fn set_http_proxy(&mut self,host_and_port:&str)->&mut Self{
        let text =format!(r#""httpProxy":"{}"}}"#,host_and_port);
        construct_the_str(&mut self.proxy_string,&text);
        self
    }
    ///Lists the address for which the proxy should be bypassed when the proxyType is "manual".
    pub fn set_no_proxy(&mut self, exceptions: Vec<&str>)->&mut Self{
        let v2s = from_vec_to_str(exceptions);
        let text =format!(r#""noProxy":{}}}"#,v2s);
        construct_the_str(&mut self.proxy_string,&text);
        self
    }
    ///Defines the proxy host for encrypted TLS traffic when the proxyType is "manual".
    pub fn set_ssl_proxy(&mut self,host_and_port:&str)->&mut Self{
        let text =format!(r#""sslProxy":"{}"}}"#,host_and_port);
        construct_the_str(&mut self.proxy_string,&text);
        self
    }
    ///Defines the proxy host for a SOCKS proxy when the proxyType is "manual".
    pub fn set_socks_proxy(&mut self,host_and_port:&str)->&mut Self{
        let text =format!(r#""socksProxy":"{}"}}"#,host_and_port);
        construct_the_str(&mut self.proxy_string,&text);
        self
    }
    ///Defines the SOCKS proxy version when the proxyType is "manual".
    pub fn set_socks_version(&mut self,version: u8)->&mut Self{
        let text = format!(r#""socksVersion":{}}}"#,version);
        construct_the_str(&mut self.proxy_string,&text);
        self
    }

}

pub enum ProxyType{
    Pac,
    Direct,
    Autodetect,
    System,
    Manual
}

fn from_vec_to_str(vec: Vec<&str>)->String{
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
fn construct_the_str(st:&mut String,text:&str){
    let len = st.len();
    if (*st).as_bytes()[len-1] == '}' as u8{
        st.pop();
        st.push(',');
        st.push_str(text);
    }else{
        st.push_str(text);
    }
}

mod prox_t{
    use super::*;

    #[test]
    fn prox_const() {
        let mut prox = Proxy::new();
        prox.set_proxy_type(ProxyType::Pac)
        .set_proxy_autoconfig_url("http://my.com")
        .set_socks_version(220)
        .set_no_proxy(vec!["one","two"])
        .set_ftp_proxy("host:port")
        .set_http_proxy("host:port")
        .set_ssl_proxy("host:port")
        .set_socks_proxy("host:port");
        let mut st = prox.proxy_string.clone();
        st.pop();
        let ch = st.pop().unwrap();
        assert!(prox.proxy_string.contains("socksVersion")&&ch!=',');
    }
}