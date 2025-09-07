use quote::format_ident;
use syn::{Ident, parse_quote, Path, Type};
use crate::kind::TypeKind;
use crate::ext::Mangle;

#[cfg(test)]
fn ident(s: &str) -> Ident {
    format_ident!("{s}")
}
impl From<&str> for TypeKind {
    fn from(s: &str) -> Self {
        TypeKind::from(&syn::parse_str::<Type>(s).unwrap())
    }
}

#[test]
fn mangle_generic_ident_test() {
    // Vec<Simple>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<u8>)),
        ident("Vec_u8")
    );
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<u32>)),
        ident("Vec_u32")
    );
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<bool>)),
        ident("Vec_bool")
    );
    // Vec<Complex>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<module::HashID>)),
        ident("Vec_module_HashID")
    );
    // Vec<Vec<Simple>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<Vec<u8>>)),
        ident("Vec_Vec_u8")
    );
    // Vec<Vec<Complex>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<Vec<module::HashID>>)),
        ident("Vec_Vec_module_HashID")
    );
    // Vec<Vec<Vec<Simple>>
    assert_eq!(
        Path::mangle_ident_default(&parse_quote!(Vec<Vec<Vec<u8>>>)),
        ident("Vec_Vec_Vec_u8")
    );
    // Vec<Vec<Vec<Complex>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<Vec<Vec<module::HashID>>>)),
        ident("Vec_Vec_Vec_module_HashID")
    );
    // Vec<Map<Simple, Simple>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(Vec<BTreeMap<u32, u32>>)),
        ident("Vec_Map_keys_u32_values_u32")
    );
    // Vec<Map<Complex, Complex>>
    assert_eq!(
        Path::mangle_ident_default(&parse_quote!(Vec<BTreeMap<module::HashID, module::KeyID>>)),
        ident("Vec_Map_keys_module_HashID_values_module_KeyID")
    );

    // Map<Simple, Simple>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<u32, u32>)),
        ident("Map_keys_u32_values_u32")
    );
    // Map<Simple, Complex>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<u32, module::HashID>)),
        ident("Map_keys_u32_values_module_HashID")
    );
    // Map<Complex, Simple>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, u32>)),
        ident("Map_keys_module_HashID_values_u32")
    );
    // Map<Complex, Complex>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, module::HashID>)),
        ident("Map_keys_module_HashID_values_module_HashID")
    );
    // Map<Complex, Vec<Simple>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, Vec<u32>>)),
        ident("Map_keys_module_HashID_values_Vec_u32")
    );
    // Map<Complex, Vec<Complex>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, Vec<module::KeyID>>)),
        ident("Map_keys_module_HashID_values_Vec_module_KeyID")
    );
    // Map<Complex, Map<Complex, Complex>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>)),
        ident("Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID")
    );
    // Map<Complex, Map<Complex, Vec<Simple>>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<u32>>>)),
        ident("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_u32")
    );
    // Map<Complex, Map<Complex, Vec<Complex>>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>)),
        ident(
            "Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID"
        )
    );
    // Map<Complex, Map<Complex, Map<Complex, Complex>>>
    assert_eq!(
        Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>>)),
        ident("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID"));
    // Map<Complex, Map<Complex, Map<Complex, Vec<Complex>>>>
    assert_eq!(
        Path::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>>)),
        ident("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID"));
}

