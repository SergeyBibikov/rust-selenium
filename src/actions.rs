use serde::{Serialize,Deserialize};
use super::specialkey::SpecialKey;

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
    pub fn add_key_actions(&mut self, key_actions:ActionsKeys){
        let temp_val = serde_json::to_string(&key_actions).unwrap();
        let mut arr:Vec<u8> = temp_val.bytes().collect();
        arr.remove(0);
        arr.pop();
        let temp_val = String::from_utf8(arr).unwrap();
        let temp_string = format!(r#"{{"type":"key",{}}}"#,temp_val);
        let val = serde_json::from_str(&temp_string).unwrap();
        self.actions.push(val);
    }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct ActionsKeys{
    actions:Vec<serde_json::Value>,
}
impl ActionsKeys{
    pub fn new()->ActionsKeys{
        ActionsKeys{
            actions: vec![],
        }
    }
    pub fn press_key(&mut self,key:&str){
       let json = format!(r#"{{"type":"keyDown","value":"{}"}}"#,key);
       let val = serde_json::from_str(&json).unwrap();
       self.actions.push(val);
    }
    pub fn release_key(&mut self,key:&str){
        let json = format!(r#"{{"type":"keyUp","value":"{}"}}"#,key);
       let val = serde_json::from_str(&json).unwrap();
       self.actions.push(val);
    }
    pub fn press_special_key(&mut self,spec_key:SpecialKey){
        let key = spec_key_to_string(spec_key);
        let json = format!(r#"{{"type":"keyDown","value":"{}"}}"#,key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
    }
    pub fn release_special_key(&mut self,spec_key:SpecialKey){
        let key = spec_key_to_string(spec_key);
        let json = format!(r#"{{"type":"keyUp","value":"{}"}}"#,key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
    }   
}
fn spec_key_to_string(spec_key:SpecialKey)->&'static str{
    match spec_key{
        SpecialKey::ShiftLeft=>r"\uE008",
        SpecialKey::ShiftRight=>r"\uE050",
        SpecialKey::LeftCtrl=>r"\uE009",
        SpecialKey::RightCtrl=>r"\uE051",
        SpecialKey::F1=>r"\uE031",
        SpecialKey::F2=>r"\uE032",
        SpecialKey::F3=>r"\uE033",
        SpecialKey::F4=>r"\uE034",
        SpecialKey::F5=>r"\uE035",
        SpecialKey::F6=>r"\uE036",
        SpecialKey::F7=>r"\uE037",
        SpecialKey::F8=>r"\uE038",
        SpecialKey::F9=>r"\uE039",
        SpecialKey::F10=>r"\uE03A",
        SpecialKey::F11=>r"\uE03B",
        SpecialKey::F12=>r"\uE03C",
        SpecialKey::EndOne=>r"\uE010",
        SpecialKey::HomeOne=>r"\uE011",
        SpecialKey::EndTwo=>r"\uE056",
        SpecialKey::HomeTwo=>r"\uE057",
        SpecialKey::PageUpOne=>r"\uE00E",
        SpecialKey::PageDownOne=>r"\uE00F",
        SpecialKey::PageUpTwo=>r"\uE054",
        SpecialKey::PageDownTwo=>r"\uE055",
        SpecialKey::OSLeft=>r"\uE03D",
        SpecialKey::OSRight=>r"\uE053",
        SpecialKey::ZenkakuHankaku=>r"\uE040",
        SpecialKey::AltLeft=>r"\uE00A",
        SpecialKey::AltRight=>r"AltRight",
        SpecialKey::ArrowLeftOne=>r"\uE012",
        SpecialKey::ArrowRightOne=>r"\uE014",
        SpecialKey::ArrowUpOne=>r"\uE013",
        SpecialKey::ArrowDownOne=>r"\uE015",
        SpecialKey::ArrowLeftTwo=>r"\uE058",
        SpecialKey::ArrowRightTwo=>r"\uE05A",
        SpecialKey::ArrowUpTwo=>r"\uE059",
        SpecialKey::ArrowDownTwo=>r"\uE05B",
        SpecialKey::InsertOne=>r"\uE016",
        SpecialKey::InsertTwo=>r"\uE05C",
        SpecialKey::DeleteOne=>r"\uE017",
        SpecialKey::DeleteTwo=>r"\uE05D",
        SpecialKey::Cancel=>r"\uE001",
        SpecialKey::Help=>r"\uE002",
        SpecialKey::Tab=>r"\uE004",
        SpecialKey::Backspace=>r"\uE003",
        SpecialKey::Clear=>r"\uE005",
        SpecialKey::Return=>r"\uE006",
        SpecialKey::Enter=>r"\uE006",
        SpecialKey::Pause=>r"\uE00B",
        SpecialKey::Escape=>r"\uE00C",
        SpecialKey::Space=>r"\uE00D",
        SpecialKey::Numpad0=>r"\uE05C",
        SpecialKey::Numpad1=>r"\uE056",
        SpecialKey::Numpad2=>r"\uE05B",
        SpecialKey::Numpad3=>r"\uE055",
        SpecialKey::Numpad4=>r"\uE058",
        SpecialKey::Numpad5=>r"\uE01F",
        SpecialKey::Numpad6=>r"\uE05A",
        SpecialKey::Numpad7=>r"\uE057",
        SpecialKey::Numpad8=>r"\uE059",
        SpecialKey::Numpad9=>r"\uE054",
        SpecialKey::NumpadAdd=>r"\uE025",
        SpecialKey::NumpadComma=>r"\uE026",
        SpecialKey::NumpadDecimal=>r"\uE05D",
        SpecialKey::NumpadDivide=>r"\uE029",
        SpecialKey::NumpadEnter=>r"\uE007",
        SpecialKey::NumpadMultiply=>r"\uE024",
        SpecialKey::NumpadSubtract=>r"\uE027",
    }
}

pub mod actions_tests{
    use super::*;
    #[test]
    #[ignore]
    fn dummy_test() {
        
        }
}