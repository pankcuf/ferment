mod constraints;
mod collection;
mod refine;
mod resolve;
mod visitor;
mod present;
mod r#abstract;
mod item;

pub use self::constraints::*;
pub use self::r#abstract::*;
pub use self::item::*;
pub use self::present::*;
pub use self::refine::*;
pub use self::resolve::*;
pub use self::visitor::*;

pub trait CrateExtension {
    fn arg_less(&self) -> Self;
    fn is_crate_based(&self) -> bool;
    fn crate_named(&self, crate_name: &Self) -> Self where Self: Sized + Clone {
        if self.is_crate_based() {
            self.replaced_first_with_ident(crate_name)
        } else {
            self.clone()
        }
    }
    fn crate_less(&self) -> Self;
    fn ident_less(&self) -> Self;
    fn crate_and_ident_less(&self) -> Self;
    fn replace_first_with(&mut self, chunk: &Self);
    fn replaced_first_with_ident(&self, chunk: &Self) -> Self;
    fn replace_last_with(&mut self, chunk: &Self);
    fn replaced_last_with(&self, chunk: &Self) -> Self;

    fn replace_first_and_last_with(&mut self, leading_chunk: &Self, trailing_chunk: &Self);
    fn replaced_first_and_last_with(&self, leading_chunk: &Self, trailing_chunk: &Self) -> Self;

}