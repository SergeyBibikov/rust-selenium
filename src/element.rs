use super::reqs::*;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

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
    pub fn find_elements_from_self(&self,locator:LocatorStrategy)->Result<Vec<Element>,String>{
        let mut result = vec![];
        let url = format!("{}/elements",self.element_url);
        let body = body_for_find_element(locator);
        let resp=send_and_read_body(Method::POST, &url, cont_length_header(&body), &body);
        if resp.contains("error"){return Err(resp);}
        let resp = parse_value(&resp);
        let map: Vec<HashMap<String,String>> = serde_json::from_str(&resp).unwrap();
        let element_ur = self.element_url.split("/element").next().unwrap();
        for i in map{
            let element_ur = element_ur.clone();
            let res = i.iter().next().unwrap();
            result.push(Element{
            element_gr_id:res.0.clone(),
            element_id:res.1.clone(),
            element_url:format!("{}/element/{}",element_ur,res.1.clone()),
            });
        }
        Ok(result)
    }
    pub fn is_selected(&self)->Result<bool,String>{
        let url = format!("{}/selected",self.element_url) ;
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,bool> = serde_json::from_str(&resp).unwrap();
        Ok(*map.get("value").unwrap())
    }
    pub fn get_attribute(&self,attribute_name: &str)->Result<String,String>{
        let url = format!("{}/attribute/{}",self.element_url,attribute_name);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        if resp.as_bytes()==br#"{"value":null}"#{return Ok("null".to_string());}
        let map:HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).clone())
    }
    ///Due to the large number of structure variants that may be returned by this function,
    /// parsing the String response to the necessary type is left for the lib users
    pub fn get_property(&self,property_name:&str)->Result<String,String>{
        let url = format!("{}/property/{}",self.element_url,property_name);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        if resp.as_bytes()==br#"{"value":null}"#{
            return Ok(String::from("null"));
        }else{Ok(resp)}

    }
    ///The logic behind returning json is the same as for get_property method
    pub fn get_css_value(&self,css_property_name:&str)->Result<String,String>{
        let url = format!("{}/css/{}",self.element_url,css_property_name);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}else{Ok(resp)}      
    }
    pub fn get_element_text(&self)->Result<String,String>{
        let url = format!("{}/text",self.element_url);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).clone())
    }
    pub fn get_tag_name(&self)->Result<String,String>{
        let url = format!("{}/name",self.element_url);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).clone())
    }
    pub fn get_element_rect(&self)->Result<ElementRect,String>{
        let url = format!("{}/rect",self.element_url);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,ElementRect> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).clone())
    }
    pub fn is_enabled(&self)->Result<bool,String>{
        let url = format!("{}/enabled",self.element_url);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,bool> = serde_json::from_str(&resp).unwrap();
        Ok(*map.get("value").unwrap())
    }
    ///As of 06.11.2020 computed role and computed label are not implemented
    /// by chrome and geckodrivers, so this method will only be returning errors for now
    pub fn get_computed_role(&self)->Result<String,String>{
        let url = format!("{}/computedrole",self.element_url);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).clone())
    }
    pub fn get_computed_label(&self)->Result<String,String>{
        let url = format!("{}/computedlabel",self.element_url);
        let resp = send_and_read_body(Method::GET, &url, vec![], "");
        if resp.contains("error"){return Err(resp);}
        let map: HashMap<&str,String> = serde_json::from_str(&resp).unwrap();
        Ok((*map.get("value").unwrap()).clone())
    }
    pub fn click(&self)->Result<(),String>{
        let body = r#"{}"#;
        let url = format!("{}/click",self.element_url);
        let resp = send_and_read_body(Method::POST, &url, cont_length_header(&body), &body);
        if resp.contains("error"){return Err(resp);}
        Ok(())

    }
    pub fn clear_element(&self){
        let url = format!("{}/clear",self.element_url);
    }
    pub fn send_keys(&self,message:&str)->Result<(),String>{
        let body = format!(r#"{{"text":"{}"}}"#,message);
        let url = format!("{}/value",self.element_url);
        let resp = send_and_read_body(Method::POST, &url, cont_length_header(&body), &body);
        if resp.contains("error"){return Err(resp);}
        Ok(())

    }
}
#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct ElementRect{
    pub(crate)height:i32,
    pub(crate)width:i32,
    pub(crate)x:i32,
    pub(crate)y:i32,
}