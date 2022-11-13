pub mod layout;

pub struct App {
    pub input: String,
    pub messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            messages: Vec::new(),
        }
    }
}

// could i just make something like styled components inside rust?
#[macro_export]
macro_rules! styled {
    ($element:expr, $props:expr, $children:expr) => {
        println!("{:?} {:?} {:?}", $element, $props, $children);
    };
}
