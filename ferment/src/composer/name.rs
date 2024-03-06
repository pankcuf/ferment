// use ferment_macro::Parent;
// use crate::composer::Composer;
// use crate::context::ScopeContext;
// use crate::naming::Name;
// use crate::shared::SharedAccess;
//
// #[derive(Parent)]
// pub struct NameComposer<'a, Parent: SharedAccess<'a>> {
//     pub parent: Option<Parent>,
//     pub name: Name,
// }
//
// impl<'a, Parent: SharedAccess<'a>> NameComposer<'a, Parent> {
//     pub const fn new(name: Name) -> Self {
//         Self { parent: None, name }
//     }
// }
//
// impl<'a, Parent: SharedAccess> Composer<Parent> for NameComposer<'a, Parent> {
//     type Source = ScopeContext;
//     type Result = Name;
//
//     fn compose(&self, _source: &Self::Source) -> Self::Result {
//         self.name.clone()
//     }
// }
//
// #[derive(Parent)]
// pub struct FFINameComposer<Parent: SharedAccess> {
//     pub parent: Option<Parent>,
//     pub name: Name,
// }
//
// impl<Parent: SharedAccess> FFINameComposer<Parent> {
//     pub const fn new(name: Name) -> Self {
//         Self { parent: None, name }
//     }
// }
//
// impl<Parent: SharedAccess> Composer<Parent> for FFINameComposer<Parent> {
//     type Source = ScopeContext;
//     type Result = Name;
//
//     fn compose(&self, _source: &Self::Source) -> Self::Result {
//         self.name.clone()
//     }
// }
