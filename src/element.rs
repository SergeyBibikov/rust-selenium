use super::reqs::*;

#[derive(Debug)]
pub struct Element{
    pub(crate)element_gr_id: String,
    pub(crate)element_id: String,
    pub(crate)element_url: String,
}
impl Element{
    pub fn find_element_from_self(&self){}
    pub fn find_elements_from_self(&self){}
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

pub mod el_tests{
    #[test]
    fn one() {
        assert!(true);
    }
}