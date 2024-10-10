use std::fmt::{Display, Formatter};
#[derive(Debug, Clone)]
pub struct Config {
    pub class_prefix: String,
    pub framework_name: String,
    pub header_name: String,
}
impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[objc::Config]\n\tframework_name: {},\n\theader_name: {},\n\tclass_prefix: {}", self.framework_name, self.header_name, self.class_prefix))
    }
}

