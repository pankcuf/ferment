use syn::{parse_quote, Path, Type};
use crate::ext::Mangle;

#[test]
fn mangle_generic_ident_test() {
    // Vec<Simple>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<u8>)).eq("Vec_u8"));
    // Vec<Simple>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<u32>)).eq("Vec_u32"));
    // Vec<Simple>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<bool>)).eq("Vec_bool"));
    // Vec<Complex>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<module::HashID>)).eq("Vec_module_HashID"));
    // Vec<Vec<Simple>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<Vec<u8>>)).eq("Vec_Vec_u8"));
    // Vec<Vec<Complex>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<Vec<module::HashID>>)).eq("Vec_Vec_module_HashID"));
    // Vec<Vec<Vec<Simple>>
    assert!(Path::mangle_ident_default(&parse_quote!(Vec<Vec<Vec<u8>>>)).eq("Vec_Vec_Vec_u8"));
    // Vec<Vec<Vec<Complex>>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<Vec<Vec<module::HashID>>>)).eq("Vec_Vec_Vec_module_HashID"));
    // Vec<Map<Simple, Simple>>
    assert!(Type::mangle_ident_default(&parse_quote!(Vec<BTreeMap<u32, u32>>)).eq("Vec_Map_keys_u32_values_u32"));
    // Vec<Map<Complex, Complex>>
    assert!(Path::mangle_ident_default(&parse_quote!(Vec<BTreeMap<module::HashID, module::KeyID>>)).eq("Vec_Map_keys_module_HashID_values_module_KeyID"));
    // Map<Simple, Simple>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<u32, u32>)).eq("Map_keys_u32_values_u32"));
    // Map<Simple, Complex>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<u32, module::HashID>)).eq("Map_keys_u32_values_module_HashID"));
    // Map<Complex, Simple>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, u32>)).eq("Map_keys_module_HashID_values_u32"));
    // Map<Complex, Complex>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, module::HashID>)).eq("Map_keys_module_HashID_values_module_HashID"));
    // Map<Complex, Vec<Simple>>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, Vec<u32>>)).eq("Map_keys_module_HashID_values_Vec_u32"));
    // Map<Complex, Vec<Complex>>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, Vec<module::KeyID>>)).eq("Map_keys_module_HashID_values_Vec_module_KeyID"));
    // Map<Complex, Map<Complex, Complex>>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>)).eq("Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID"));
    // Map<Complex, Map<Complex, Vec<Simple>>>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<u32>>>)).eq("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_u32"));
    // Map<Complex, Map<Complex, Vec<Complex>>>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>)).eq("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID"));
    // Map<Complex, Map<Complex, Map<Complex, Complex>>>
    assert!(Type::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>>)).eq("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID"));
    // Map<Complex, Map<Complex, Map<Complex, Vec<Complex>>>>
    assert!(Path::mangle_ident_default(&parse_quote!(BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>>)).eq("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID"));
}

