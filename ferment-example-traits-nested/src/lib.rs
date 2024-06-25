mod fermented;
// mod model;

extern crate ferment_macro;


pub mod some_package {
    use ferment_example_traits::from_proof::from_proof::FromProof;

    #[ferment_macro::export]
    pub trait SomeTrait where Self: FromProof {}


    // #[ferment_macro::export]
    // pub struct SomeGenericStruct<Req, FP> where FP: FromProof<Req> {
    //     pub obj: FP,
    // }
}