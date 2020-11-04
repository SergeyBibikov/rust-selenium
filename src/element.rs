use super::reqs::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Element{
    pub(crate)element_gr_id: String,
    pub(crate)element_id: String,
    pub(crate)element_url: String,
}
impl Element{
    pub fn find_element_from_self(&self,locator:LocatorStrategy)->Result<Element,String>{
        let url = format!("{}/element",self.element_url);
        let body = body_for_find_element(locator);
        let resp = send_and_read_body(Method::POST, &url, cont_length_header(&body), &body);
        if resp.contains("error"){return Err(resp);}
        let resp = parse_value(&resp);
        let map: HashMap<String,String> = serde_json::from_str(&resp).unwrap();
        let res = map.iter().next().unwrap();
        let el_url = self.element_url.split("/element").next().unwrap();
        Ok(Element{
            element_gr_id:res.0.clone(),
            element_id:res.1.clone(),
            element_url: format!("{}/element/{}",el_url,res.1.clone()),
        })
    }
    // pub fn find_elements_from_self(&self,locator:LocatorStrategy)->Result<Vec<Element>,String>{

    // }
    pub fn is_selected(&self){}
    pub fn get_attribute(&self){}
    pub fn get_property(&self){}
    pub fn get_css_value(&self){}
    pub fn get_element_text(&self){}
    pub fn get_tag_name(&self){}
    pub fn get_element_rect(&self){}
    pub fn is_enabled(&self){}
    pub fn get_computed_role(&self){}
    pub fn get_computed_label(&self){}
    pub fn click(&self){}
    pub fn clear_element(&self){}
    pub fn send_keys(&self){}
}
/*
pub mod el_tests{
    #[test]
    fn one() {
        assert!(true);
    }
}*/