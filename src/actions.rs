use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Actions{
    actions:Vec<serde_json::Value>,
}
impl Actions{
    pub fn new()->Actions{
        Actions{
            actions: vec![],
        }
    }
    pub(crate) fn set_ids(&mut self){
        let mut act_with_ids = vec![];
        let len = self.actions.len();
        for i in 0..len{
            let mut st = serde_json::to_string(&self.actions[i]).unwrap();
            st.pop();
            let id = format!(r#","id":"{}"}}"#,i+1);
            st.push_str(&id);
            let v = serde_json::from_str(&st).unwrap();
            act_with_ids.push(v);
        }
        self.actions = act_with_ids;
    }
    pub fn release_focus(&mut self){
        let a = serde_json::json!({"type":"pointer", "actions":[{"type":"pointerDown","button":0}]});
        self.actions.push(a);
    }
    pub fn ctrl_a(&mut self){
        let a = serde_json::from_str(r#"{"type":"key", "actions":[
                                    {"type":"keyDown","value":"\uE009"},
                                    {"type":"keyDown","value":"a"},
                                    {"type":"keyUp","value":"\uE009"}]}"#).unwrap();
        self.actions.push(a);
    }
    pub fn ctrl_c(&mut self){
        let a = serde_json::from_str(r#"{"type":"key", "actions":[
                                    {"type":"keyDown","value":"\uE009"},
                                    {"type":"keyDown","value":"c"},
                                    {"type":"keyUp","value":"\uE009"}]}"#).unwrap();
        self.actions.push(a);
    }
    pub fn ctrl_v(&mut self){
        let a = serde_json::from_str(r#"{"type":"key", "actions":[
                                    {"type":"keyDown","value":"\uE009"},
                                    {"type":"keyDown","value":"v"},
                                    {"type":"keyUp","value":"\uE009"}]}"#).unwrap();
        self.actions.push(a);
    }
}

pub mod actions_tests{
    use super::*;
#[test]
fn dummy_test() {
    
}
}