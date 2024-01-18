
#[derive(Clone, Debug, Default)]
pub struct Context {
    pub crate_names: Vec<String>,
    // pub traits: HashMap<Scope, >
    pub mod_name: String,
}

// impl Context {
//     pub fn new(crate_names: Vec<String>) -> Self {
//         Self { crate_names }
//     }
//
//     pub fn contains_fermented_crate(&self, ident: &Ident) -> bool {
//         self.crate_names.contains(&ident.to_string())
//     }
//
//     pub(crate) fn merge(&mut self, context: &Context) {
//         self.crate_names = context.crate_names.clone();
//     }
// }