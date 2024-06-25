mod iterative;
mod sequence;
mod sequence_mixer;
mod linked;
mod context;

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
