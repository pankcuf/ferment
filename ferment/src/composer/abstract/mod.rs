mod composable;
mod context;
mod iterative;
mod linked;
mod sequence;
mod sequence_mixer;

#[allow(unused)]
mod new;
#[allow(unused)]
mod new_const;

pub use self::composable::*;
pub use self::context::*;
pub use self::iterative::*;
pub use self::linked::*;
pub use self::sequence::*;
pub use self::sequence_mixer::*;

pub trait Composer<'a> {
    type Source;
    type Result;
    fn compose(&self, source: &'a Self::Source) -> Self::Result;
}

