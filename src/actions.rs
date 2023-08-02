use super::element::*;
use super::specialkey::*;
use serde::{Deserialize, Serialize};

///Main actions struct
///
/// The Actions which should be permormed via the perform_actions and release_actions methods of the Browser instance,
/// may consist of keys actions, scroll wheel actions and mouse actions. Once you construct the corresponding actions sequence
/// it should be passed to the add_..._actions method to be added to the main Actions instance.
/// When constructing complex scenarious with multiple input sources(key,mouse,wheel), you need to use pauses
/// to syncronize actions and achieve more predictable result
/// (see "https://www.w3.org/TR/webdriver/#actions" for more details).
///
/// The order of the adding input source to Actions may also influence the outcome.
///
/// # Examples
/// ```
/// # use selenium_webdriver::*;
/// let mut mouse = ActionsMouse::new();
/// let mut keys = ActionsKeys::new();
/// mouse.press_mouse_button(MouseButton::Left).pause(0).release_mouse_button(MouseButton::Left);
/// keys.pause(0).press_special_key(SpecialKey::Enter);
/// let mut actions = Actions::new();
/// actions.add_mouse_actions(mouse).add_key_actions(keys);
/// ```

#[derive(Serialize, Deserialize, Debug)]
pub struct Actions {
    pub(crate) actions: Vec<serde_json::Value>,
}

impl Actions {
    pub fn new() -> Actions {
        Actions { actions: vec![] }
    }
    pub(crate) fn set_ids(&mut self) {
        let mut act_with_ids = vec![];
        let len = self.actions.len();
        for i in 0..len {
            let mut st = serde_json::to_string(&self.actions[i]).unwrap();
            st.pop();
            let id = format!(r#","id":"{}"}}"#, i + 1);
            st.push_str(&id);
            let v = serde_json::from_str(&st).unwrap();
            act_with_ids.push(v);
        }
        self.actions = act_with_ids;
    }
    /// Add the key actions sequence to the global list of actions
    ///
    /// # Examples
    ///
    /// ```
    /// # use selenium_webdriver::*;
    ///
    /// let mut keys = ActionsKeys::new();
    /// keys.press_special_key(SpecialKey::ShiftLeft).press_key("a");
    /// let mut actions = Actions::new();
    /// actions.add_key_actions(keys);
    /// ```
    pub fn add_key_actions(&mut self, key_actions: ActionsKeys) -> &mut Self {
        let temp_val = serde_json::to_string(&key_actions).unwrap();
        let mut arr: Vec<u8> = temp_val.bytes().collect();
        arr.remove(0);
        arr.pop();
        let temp_val = String::from_utf8(arr).unwrap();
        let temp_string = format!(r#"{{"type":"key",{}}}"#, temp_val);
        let val = serde_json::from_str(&temp_string).unwrap();
        self.actions.push(val);
        self
    }
    pub fn add_mouse_actions(&mut self, mouse_actions: ActionsMouse) -> &mut Self {
        let temp_val = serde_json::to_string(&mouse_actions).unwrap();
        let mut arr: Vec<u8> = temp_val.bytes().collect();
        arr.remove(0);
        arr.pop();
        let temp_val = String::from_utf8(arr).unwrap();
        let temp_string = format!(r#"{{"type":"pointer",{}}}"#, temp_val);
        let val = serde_json::from_str(&temp_string).unwrap();
        self.actions.push(val);
        self
    }
    pub fn add_wheel_actions(&mut self, wheel_actions: ActionsWheel) -> &mut Self {
        let temp_val = serde_json::to_string(&wheel_actions).unwrap();
        let mut arr: Vec<u8> = temp_val.bytes().collect();
        arr.remove(0);
        arr.pop();
        let temp_val = String::from_utf8(arr).unwrap();
        let temp_string = format!(r#"{{"type":"wheel",{}}}"#, temp_val);
        let val = serde_json::from_str(&temp_string).unwrap();
        self.actions.push(val);
        self
    }
}
///Struct to create the key actions sequence
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsKeys {
    pub(crate) actions: Vec<serde_json::Value>,
}
impl ActionsKeys {
    pub fn new() -> ActionsKeys {
        ActionsKeys { actions: vec![] }
    }
    pub fn press_key(&mut self, key: &str) -> &mut Self {
        let json = format!(r#"{{"type":"keyDown","value":"{}"}}"#, key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    pub fn release_key(&mut self, key: &str) -> &mut Self {
        let json = format!(r#"{{"type":"keyUp","value":"{}"}}"#, key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    pub fn press_special_key(&mut self, spec_key: SpecialKey) -> &mut Self {
        let key = spec_key_to_string(spec_key);
        let json = format!(r#"{{"type":"keyDown","value":"{}"}}"#, key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    pub fn release_special_key(&mut self, spec_key: SpecialKey) -> &mut Self {
        let key = spec_key_to_string(spec_key);
        let json = format!(r#"{{"type":"keyUp","value":"{}"}}"#, key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    pub fn pause(&mut self, duration: u32) -> &mut Self {
        let json = format!(r#"{{"type":"pause","duration":{}}}"#, duration);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
}
fn spec_key_to_string(spec_key: SpecialKey) -> &'static str {
    match spec_key {
        SpecialKey::ShiftLeft => r"\uE008",
        SpecialKey::ShiftRight => r"\uE050",
        SpecialKey::LeftCtrl => r"\uE009",
        SpecialKey::RightCtrl => r"\uE051",
        SpecialKey::F1 => r"\uE031",
        SpecialKey::F2 => r"\uE032",
        SpecialKey::F3 => r"\uE033",
        SpecialKey::F4 => r"\uE034",
        SpecialKey::F5 => r"\uE035",
        SpecialKey::F6 => r"\uE036",
        SpecialKey::F7 => r"\uE037",
        SpecialKey::F8 => r"\uE038",
        SpecialKey::F9 => r"\uE039",
        SpecialKey::F10 => r"\uE03A",
        SpecialKey::F11 => r"\uE03B",
        SpecialKey::F12 => r"\uE03C",
        SpecialKey::EndOne => r"\uE010",
        SpecialKey::HomeOne => r"\uE011",
        SpecialKey::EndTwo => r"\uE056",
        SpecialKey::HomeTwo => r"\uE057",
        SpecialKey::PageUpOne => r"\uE00E",
        SpecialKey::PageDownOne => r"\uE00F",
        SpecialKey::PageUpTwo => r"\uE054",
        SpecialKey::PageDownTwo => r"\uE055",
        SpecialKey::OSLeft => r"\uE03D",
        SpecialKey::OSRight => r"\uE053",
        SpecialKey::ZenkakuHankaku => r"\uE040",
        SpecialKey::AltLeft => r"\uE00A",
        SpecialKey::AltRight => r"AltRight",
        SpecialKey::ArrowLeftOne => r"\uE012",
        SpecialKey::ArrowRightOne => r"\uE014",
        SpecialKey::ArrowUpOne => r"\uE013",
        SpecialKey::ArrowDownOne => r"\uE015",
        SpecialKey::ArrowLeftTwo => r"\uE058",
        SpecialKey::ArrowRightTwo => r"\uE05A",
        SpecialKey::ArrowUpTwo => r"\uE059",
        SpecialKey::ArrowDownTwo => r"\uE05B",
        SpecialKey::InsertOne => r"\uE016",
        SpecialKey::InsertTwo => r"\uE05C",
        SpecialKey::DeleteOne => r"\uE017",
        SpecialKey::DeleteTwo => r"\uE05D",
        SpecialKey::Cancel => r"\uE001",
        SpecialKey::Help => r"\uE002",
        SpecialKey::Tab => r"\uE004",
        SpecialKey::Backspace => r"\uE003",
        SpecialKey::Clear => r"\uE005",
        SpecialKey::Return => r"\uE006",
        SpecialKey::Enter => r"\uE006",
        SpecialKey::Pause => r"\uE00B",
        SpecialKey::Escape => r"\uE00C",
        SpecialKey::Space => r"\uE00D",
        SpecialKey::Numpad0 => r"\uE05C",
        SpecialKey::Numpad1 => r"\uE056",
        SpecialKey::Numpad2 => r"\uE05B",
        SpecialKey::Numpad3 => r"\uE055",
        SpecialKey::Numpad4 => r"\uE058",
        SpecialKey::Numpad5 => r"\uE01F",
        SpecialKey::Numpad6 => r"\uE05A",
        SpecialKey::Numpad7 => r"\uE057",
        SpecialKey::Numpad8 => r"\uE059",
        SpecialKey::Numpad9 => r"\uE054",
        SpecialKey::NumpadAdd => r"\uE025",
        SpecialKey::NumpadComma => r"\uE026",
        SpecialKey::NumpadDecimal => r"\uE05D",
        SpecialKey::NumpadDivide => r"\uE029",
        SpecialKey::NumpadEnter => r"\uE007",
        SpecialKey::NumpadMultiply => r"\uE024",
        SpecialKey::NumpadSubtract => r"\uE027",
    }
}
///Struct to create the mouse actions sequence
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsMouse {
    pub(crate) actions: Vec<serde_json::Value>,
}
impl ActionsMouse {
    pub fn new() -> ActionsMouse {
        ActionsMouse { actions: vec![] }
    }
    pub fn pause(&mut self, duration: u32) -> &mut Self {
        let json = format!(r#"{{"type":"pause","duration":{}}}"#, duration);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    pub fn press_mouse_button(&mut self, button: MouseButton) -> &mut Self {
        let key = mouse_button_to_string(button);
        let json = format!(r#"{{"type":"pointerDown","button":{}}}"#, key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    pub fn release_mouse_button(&mut self, button: MouseButton) -> &mut Self {
        let key = mouse_button_to_string(button);
        let json = format!(r#"{{"type":"pointerUp","button":{}}}"#, key);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    /// The point's coordinates are relative to the viewport, if x or y is larger
    /// than the coordinate of the viewport, you will get an error calling
    /// the perform_actions method of the Browser
    pub fn move_mouse_to_point(&mut self, x: i32, y: i32) -> &mut Self {
        let json = format!(
            r#"{{"type":"pointerMove","duration":0,"origin":"viewport","x":{},"y":{}}}"#,
            x, y
        );
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    /// Moves mouse to the center of the element. Will cause an error if the element is not in the viewport
    pub fn move_mouse_to_element(&mut self, element: &Element) -> &mut Self {
        let el = element.get_element_rect().unwrap();
        self.move_mouse_to_point(el.x as i32 + el.width / 2, el.y as i32 + el.height / 2);
        self
    }
    pub fn cancel_action(&mut self) -> &mut Self {
        let json = r#"{"type":"pointerCancel"}"#;
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    ///Drag one element and drop it on another one.
    pub fn drag_n_drop(&mut self, elem_to_drag: Element, elem_destination: Element) -> &mut Self {
        self.move_mouse_to_element(&elem_to_drag)
            .press_mouse_button(MouseButton::Left)
            .move_mouse_to_element(&elem_destination)
            .release_mouse_button(MouseButton::Left);
        self
    }
}
fn mouse_button_to_string(button: MouseButton) -> u8 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
        MouseButton::X1Back => 3,
        MouseButton::X2Forward => 4,
    }
}
///Struct to create the wheel actions sequence
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionsWheel {
    pub(crate) actions: Vec<serde_json::Value>,
}
impl ActionsWheel {
    pub fn new() -> ActionsWheel {
        ActionsWheel { actions: vec![] }
    }
    pub fn pause(&mut self, duration: u32) -> &mut Self {
        let json = format!(r#"{{"type":"pause","duration":{}}}"#, duration);
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
    ///Scroll by number of pixels (x-axis and y-axis) from a starting position within the viewport.
    ///The value of pixels to scroll may be both positive and negative.
    /// # Examples
    /// ```
    /// # use selenium_webdriver::*;
    /// let mut wheel = ActionsWheel::new();
    /// wheel.scroll(0, 0, 100, 100).scroll(100, 100, 100, 100);
    /// ```
    pub fn scroll(
        &mut self,
        init_x_position: i32,
        init_y_position: i32,
        x_axis_scroll: i32,
        y_axis_scroll: i32,
    ) -> &mut Self {
        let json = format!(
            r#"{{"type":"scroll",
                            "x":{},
                            "y":{},
                            "deltaX":{},
                            "deltaY":{}}}"#,
            init_x_position, init_y_position, x_axis_scroll, y_axis_scroll
        );
        let val = serde_json::from_str(&json).unwrap();
        self.actions.push(val);
        self
    }
}

mod actions_tests {
    use super::*;
    #[test]
    fn actions_vec_lens() {
        let mut ac = Actions::new();
        let mut ma = ActionsMouse::new();
        ma.press_mouse_button(MouseButton::Left);
        ma.pause(5);
        ma.press_mouse_button(MouseButton::Right);
        assert!(ma.actions.len() == 3);
        ac.add_mouse_actions(ma);
        assert!(ac.actions.len() == 1);
    }
}
