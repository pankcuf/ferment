use crate::aa::AtCc;

#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct AtWw2 {
    pub version: u32,
    pub vec_cc: Vec<AtCc>,
}