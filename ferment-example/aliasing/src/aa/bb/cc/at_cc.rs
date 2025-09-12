use crate::aa::{AtAa, AtBb, AtDd};
use crate::zz::at_zz::AtZz;
use crate::zz::xx::ww::{AtWw, AtWw2};
use crate::zz::yy::at_yy::AtYy;
use crate::zz::yy::xx::at_xx::AtXx;

#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct AtCc {
    pub aa: AtAa,
    pub bb: AtBb,
    pub dd: AtDd,
    pub xx: AtXx,
    pub yy: AtYy,
    pub zz: AtZz,
    pub vec_ww: Vec<AtWw>,
    pub vec_ww2: Vec<AtWw2>,
}