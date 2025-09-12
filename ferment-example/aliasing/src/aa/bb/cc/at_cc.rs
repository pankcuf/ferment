use crate::aa::{AtAa, AtBb, AtDd};
use crate::zz::at_zz::AtZz;
use crate::zz::yy::at_yy::AtYy;
use crate::zz::yy::xx::at_xx::AtXx;
use crate::zz::yy::xx::ww::at_ww::AtWw;

#[derive(Clone, Debug)]
#[ferment_macro::export]
pub struct AtCc {
    pub aa: AtAa,
    pub bb: AtBb,
    pub dd: AtDd,
    pub ww: AtWw,
    pub xx: AtXx,
    pub yy: AtYy,
    pub zz: AtZz,
}