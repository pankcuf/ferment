use proc_macro2::Ident;
use syn::{Path, PathSegment};
use crate::ast::Colon2Punctuated;

pub trait DictionaryType {

    fn is_primitive(&self) -> bool {
        self.is_digit() || self.is_bool()
    }

    fn is_any_string(&self) -> bool {
        self.is_str() || self.is_string()
    }
    fn is_void(&self) -> bool;
    fn is_digit(&self) -> bool;
    fn is_128_digit(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_str(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_vec(&self) -> bool;
    fn is_smart_ptr(&self) -> bool;
    fn is_special_std_trait(&self) -> bool;
    fn is_special_generic(&self) -> bool;
    fn is_result(&self) -> bool;
    fn is_map(&self) -> bool;
    fn is_box(&self) -> bool;
    fn is_optional(&self) -> bool;
    fn is_lambda_fn(&self) -> bool;
    // fn is_from(&self) -> bool;
    // fn is_into(&self) -> bool;
}

impl DictionaryType for Ident {
    fn is_void(&self) -> bool {
        self.to_string().eq("c_void")
    }

    fn is_digit(&self) -> bool {
        matches!(self.to_string().as_str(), "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize" | "usize")
    }

    // 128-bit integers don't currently have a known stable ABI so they aren't FFI-safe, should be exported as [u8/i8; 16] instead
    fn is_128_digit(&self) -> bool {
        matches!(self.to_string().as_str(), "i128" | "u128")
    }

    fn is_bool(&self) -> bool {
        self.to_string().as_str() == "bool"
    }
    fn is_str(&self) -> bool {
        self.to_string().as_str() == "str"
    }

    fn is_string(&self) -> bool {
        self.to_string().as_str() == "String"
    }
    fn is_vec(&self) -> bool {
        self.to_string().as_str() == "Vec"
    }

    fn is_smart_ptr(&self) -> bool {
        self.is_box() || matches!(self.to_string().as_str(), "Arc" | "Rc" | "Cell" | "RefCell" | "Mutex" | "RwLock")
    }

    fn is_special_std_trait(&self) -> bool {
        matches!(self.to_string().as_str(), "Send" | "Sync" | "Clone" | "Sized")
    }
    fn is_special_generic(&self) -> bool {
        self.is_map() || self.is_vec() || matches!(self.to_string().as_str(), "IndexMap" | "BTreeSet" | "HashSet")
    }

    fn is_result(&self) -> bool {
        matches!(self.to_string().as_str(), "Result")
    }

    fn is_map(&self) -> bool {
        matches!(self.to_string().as_str(), "BTreeMap" | "HashMap")
    }

    fn is_box(&self) -> bool {
        matches!(self.to_string().as_str(), "Box")
    }

    fn is_optional(&self) -> bool {
        matches!(self.to_string().as_str(), "Option")
    }

    fn is_lambda_fn(&self) -> bool {
        matches!(self.to_string().as_str(), "FnOnce" | "Fn" | "FnMut")
    }

    // fn is_from(&self) -> bool {
    //     matches!(self.to_string().as_str(), "From")
    // }
    //
    // fn is_into(&self) -> bool {
    //     matches!(self.to_string().as_str(), "Into")
    // }
}

impl DictionaryType for PathSegment {
    fn is_void(&self) -> bool {
        self.ident.is_void()
    }

    fn is_digit(&self) -> bool {
        self.ident.is_digit()
    }

    // 128-bit integers don't currently have a known stable ABI so they aren't FFI-safe, should be exported as [u8/i8; 16] instead
    fn is_128_digit(&self) -> bool {
        self.ident.is_128_digit()
    }

    fn is_bool(&self) -> bool {
        self.ident.is_bool()
    }
    fn is_str(&self) -> bool {
        self.ident.is_str()
    }

    fn is_string(&self) -> bool {
        self.ident.is_string()
    }
    fn is_vec(&self) -> bool {
        self.ident.is_vec()
    }

    fn is_smart_ptr(&self) -> bool {
        self.ident.is_smart_ptr()
    }

    fn is_special_std_trait(&self) -> bool {
        self.ident.is_special_std_trait()
    }
    fn is_special_generic(&self) -> bool {
        self.ident.is_special_generic()
    }

    fn is_result(&self) -> bool {
        self.ident.is_result()
    }

    fn is_map(&self) -> bool {
        self.ident.is_map()
    }

    fn is_box(&self) -> bool {
        self.ident.is_box()
    }

    fn is_optional(&self) -> bool {
        self.ident.is_optional()
    }

    fn is_lambda_fn(&self) -> bool {
        self.ident.is_lambda_fn()
    }
}
impl DictionaryType for Colon2Punctuated<PathSegment> {
    fn is_void(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_void())
    }
    fn is_digit(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_digit())
    }
    fn is_128_digit(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_128_digit())
    }

    fn is_bool(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_bool())
    }
    fn is_str(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_str())
    }

    fn is_string(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_string())
    }
    fn is_vec(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_vec())
    }

    fn is_smart_ptr(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_smart_ptr())
    }

    fn is_special_std_trait(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_special_std_trait())
    }
    fn is_special_generic(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_special_generic())
    }

    fn is_result(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_result())
    }

    fn is_map(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_map())
    }

    fn is_box(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_box())
    }

    fn is_optional(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_optional())
    }

    fn is_lambda_fn(&self) -> bool {
        self.last().map_or(false, |seg| seg.is_lambda_fn())
    }
}

impl DictionaryType for Path {
    fn is_void(&self) -> bool {
        self.segments.is_void()
    }
    fn is_digit(&self) -> bool {
        self.segments.is_digit()
    }
    fn is_128_digit(&self) -> bool {
        self.segments.is_128_digit()
    }

    fn is_bool(&self) -> bool {
        self.segments.is_bool()
    }
    fn is_str(&self) -> bool {
        self.segments.is_str()
    }

    fn is_string(&self) -> bool {
        self.segments.is_string()
    }
    fn is_vec(&self) -> bool {
        self.segments.is_vec()
    }

    fn is_smart_ptr(&self) -> bool {
        self.segments.is_smart_ptr()
    }

    fn is_special_std_trait(&self) -> bool {
        self.segments.is_special_std_trait()
    }
    fn is_special_generic(&self) -> bool {
        self.segments.is_special_generic()
    }

    fn is_result(&self) -> bool {
        self.segments.is_result()
    }

    fn is_map(&self) -> bool {
        self.segments.is_map()
    }

    fn is_box(&self) -> bool {
        self.segments.is_box()
    }

    fn is_optional(&self) -> bool {
        self.segments.is_optional()
    }

    fn is_lambda_fn(&self) -> bool {
        self.segments.is_lambda_fn()
    }

}