#[derive(Clone, Debug)]
#[allow(unused)]
pub enum ScopePropagation {
    None = 0,
    Local = 1,
    Global = 2,
    GlobalAndLocal = 3,
}