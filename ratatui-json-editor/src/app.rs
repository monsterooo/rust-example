/// 三个主要屏幕，Main 显示已存在的值屏幕，Editing 显示创建屏幕，Exiting 已存在提示屏幕
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

/// 追踪用户当前正在输入的字段
pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,              // 当前编辑的json key
    pub value_input: String,            // 当前编辑的 json value
    pub pairs: HashMap<String, String>, // serde支持的序列化键值对表示
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>, // 正在编辑的键值可选状态，当用户不是编辑时为None
}

impl App {
    pub fn new() -> Self {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
}
