// use quote::{format_ident, ToTokens};
// use syn::{parse_quote, Path, PathSegment};
// use crate::ast::Colon2Punctuated;
// use crate::ext::ReexportSeek;

// #[test]
// fn imports() {
//     // (Scope, Import, FullImport)
//     let crate_ident = format_ident!("aa");
//     let samples: Vec<(Path, Path, Colon2Punctuated<PathSegment>)> = vec![
//         (parse_quote!(aa::bb::cc::dd), parse_quote!(crate::xx::Ident), parse_quote!(aa::xx::Ident)),
//         (parse_quote!(aa::bb::cc), parse_quote!(super::xx::Ident), parse_quote!(aa::bb::xx::Ident)),
//         (parse_quote!(aa::bb::cc::dd), parse_quote!(super::xx::Ident), parse_quote!(aa::bb::cc::xx::Ident)),
//         (parse_quote!(aa::bb::cc::dd), parse_quote!(super::super::xx::Ident), parse_quote!(aa::bb::xx::Ident)),
//         (parse_quote!(aa::bb::cc::dd), parse_quote!(super::super::super::xx::Ident), parse_quote!(aa::xx::Ident)),
//         (parse_quote!(aa::bb::cc::dd), parse_quote!(self::xx::Ident), parse_quote!(aa::bb::cc::dd::xx::Ident)),
//     ];
//     for (scope, import, full_import) in samples {
//         let result = ReexportSeek::Any.join_reexport(&import, &scope, &crate_ident, None);
//         println!("check: {}", result.to_token_stream());
//         assert_eq!(result, full_import)
//     }
// }

// REFINE: Imported($Ty(Runtime, []), tokio :: runtime :: Runtime) in [ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime(Object + Opaque)]
// maybe_item: tokio::runtime::Runtime
// ReexportSeek::Any (Not Found)
// (Imported) (not found): tokio :: runtime :: Runtime
// traverse_scopes_for_import (check):
// $Ty(tokio :: runtime :: Runtime, []) in
// [ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime(Object + Opaque)]
// maybe_scope_item_conversion ...
// $Ty(tokio :: runtime :: Runtime, []) in
// ferment_example_entry_point :: entry :: rnt :: tokio :: runtime :: Runtime
// maybe_item: ferment_example_entry_point :: entry :: rnt :: tokio :: runtime :: Runtime
// traverse_scopes_for_import (not found in parent): ferment_example_entry_point :: entry :: rnt :: tokio :: runtime :: Runtime
// traverse_scopes_for_import (reexport found): ferment_example_entry_point :: entry :: rnt :: tokio :: runtime :: Runtime
// maybe_scope_item_conversion ...
// $Ty(tokio :: runtime :: Runtime, []) in
// ferment_example_entry_point :: entry :: rnt :: tokio :: runtime :: Runtime
// maybe_item: ferment_example_entry_point :: entry :: rnt :: tokio :: runtime :: Runtime
// traverse_scopes_for_import (222): ferment_example_entry_point :: entry :: rnt :: DashSharedCoreWithRuntime
// maybe_item: ferment_example_entry_point :: entry :: rnt :: DashSharedCoreWithRuntime
// [INFO] Found item in scope: Item(DashSharedCoreWithRuntime, ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime)
// traverse_scopes (found): $Ty(tokio :: runtime :: Runtime, []) in Item(DashSharedCoreWithRuntime, ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime)
// update_scope_item: Item(DashSharedCoreWithRuntime, ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime) --- $Ty(tokio :: runtime :: Runtime, [])
// ScopeItemConversion::update_scope_item NEW_OBJECT.1: $Ty(tokio :: runtime :: Runtime, [])
// [INFO] Known item found: [Object($Ty(tokio :: runtime :: Runtime, []))]
// maybe_refined_object.2: Type(Object($Ty(tokio :: runtime :: Runtime, []))) in [ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime(Object + Opaque)]
// maybe_refined_object.2: None in [ferment_example_entry_point::entry::SomeModel(Object + Fermented)]

// REFINE Import: tokio::runtime::Runtime in ferment_example_entry_point::entry::rnt::DashSharedCoreWithRuntime(Object + Opaque)
// ScopeChain::Object
//  -- Import Scope: [ferment_example_entry_point::entry::rnt]
//      -- Has Scope?: ferment_example_entry_point::entry::rnt::tokio::runtime::Runtime --- No
//      -- Has Scope? ferment_example_entry_point::entry::rnt::tokio::runtime --- No
//      -- Has Scope? ferment_example_entry_point::entry::rnt::tokio --- No
//      -- Not a local import, so check globals:
//          -- Has Scope? tokio::runtime --- No
//          -- Has Scope? tokio --- No
//          -- Not a global import, so it's from non-fermented crate -> So it's opaque