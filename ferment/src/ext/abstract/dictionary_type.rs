use proc_macro2::Ident;

pub trait DictionaryType {

    fn is_primitive(&self) -> bool {
        self.is_digit() || self.is_bool()
    }
    fn is_any_string(&self) -> bool {
        self.is_str() || self.is_string()
    }
    fn is_digit(&self) -> bool;
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
    fn is_digit(&self) -> bool {
        matches!(self.to_string().as_str(), "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f64" | "i128" | "u128" | "isize" | "usize")
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