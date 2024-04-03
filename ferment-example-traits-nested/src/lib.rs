mod fermented;
mod model;

extern crate ferment_macro;

#[ferment_macro::export]
pub struct SomeStruct {
    pub name: String,
}

pub mod some_package {
    use ferment_example_traits::from_proof::from_proof::FromProof;

    #[ferment_macro::export]
    pub trait SomeTrait<Req> where Self: FromProof<Req> {
    }


    // #[ferment_macro::export]
    // pub struct SomeGenericStruct<Req, FP> where FP: FromProof<Req> {
    //     pub obj: FP,
    // }
}