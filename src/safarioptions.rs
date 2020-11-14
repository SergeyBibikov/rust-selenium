///Utility struct to adjust the safari browser session
///
///  Please see https://developer.apple.com/documentation/webkit/about_webdriver_for_safari for more info
pub struct SafariOptions{
    pub (crate) base_string: String,
}
impl SafariOptions{
    pub fn new()->Self{
        SafariOptions{
            base_string:String::new()
        }
    }
    ///Preloads the Web Inspector and JavaScript debugger in the background.
    pub fn enable_automatic_inspection(&mut self)->&mut Self{
        if self.base_string.contains("Inspection"){panic!("The automaticInspection is already enabled!");}
        let text = r#""safari:automaticInspection":true"#;
        update_string(&mut self.base_string,text);
        self
    }
    ///Preloads Web Inspector and starts a timeline recording in the background.
    pub fn enable_automatic_profiling(&mut self)->&mut Self{
        if self.base_string.contains("Profiling"){panic!("The automaticProfiling is already enabled!");}
        let text = r#""safari:automaticProfiling":true"#;
        update_string(&mut self.base_string,text);
        self
    }
    ///Turns on the debugger for safaridriver
    pub fn enable_diagnose(&mut self)->&mut Self{
        if self.base_string.contains("diagnose"){panic!("The diagnostics are already enabled!");}
        let text = r#""safari:diagnose":true"#;
        update_string(&mut self.base_string,text);
        self
    }
    ///Turn on the IOS simulator
    pub fn enable_simulator(&mut self)->&mut Self{
        if self.base_string.contains("Simulator"){panic!("The iOS simulation is already enabled!");}
        let text = r#""safari:useSimulator":true"#;
        update_string(&mut self.base_string,text);
        self
    }
}
// makes the dirty work with base string
fn update_string(st: &mut String,text:&str){
    let comma = ',' as u8;
    let len = st.len();
    if len>0{
        st.push(',');
        st.push_str(text);
    }else{
        st.push_str(text);
    }
    /*if (*st).as_bytes()[len-1]==comma{
        st.push_str(text);
    }else if len{
        st.pop();
        st.pop();
        st.pop();
        st.push(',');
        st.push('\n');
        st.push_str(text);
    }*/
}

mod saf_gen{
    use super::*;
    #[test]
    fn saf_base_str() {
        let mut saf = SafariOptions::new();
        saf.enable_simulator().enable_automatic_inspection().enable_automatic_profiling().enable_diagnose();
        println!("{}",saf.base_string);
        assert!(saf.base_string.contains("safari:diagnose")&&
        saf.base_string.contains("safari:automaticInspection")&&
        saf.base_string.contains("safari:automaticProfiling")&&
        saf.base_string.contains("safari:useSimulator"));
    }
    #[test]
    fn name() {
        let mut saf = SafariOptions::new();
        saf.enable_simulator();
        println!("{}",saf.base_string);
    }
    #[test]
    #[should_panic]
    fn saf_double_add_sim() {
        let mut saf = SafariOptions::new();
        saf.enable_simulator().enable_simulator();
    }
}