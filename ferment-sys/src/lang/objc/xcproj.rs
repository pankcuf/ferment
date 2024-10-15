use std::fmt::{Display, Formatter};
#[derive(Debug, Clone)]
pub struct Config {
    pub class_prefix: String,
    pub framework_name: String,
    pub header_name: String,
}
impl Config {
    pub fn new(class_prefix: &str, framework_name: &str, header_name: &str) -> Self {
        Self { class_prefix: String::from(class_prefix), framework_name: String::from(framework_name), header_name: String::from(header_name) }
    }
}
impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[objc::Config]\n\tframework_name: {},\n\theader_name: {},\n\tclass_prefix: {}", self.framework_name, self.header_name, self.class_prefix))
    }
}

