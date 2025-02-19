pub const MACROS: &str = r#"
#define FFIGroupConversion(TYPE, VarValue, FromValue, ToValue, DestroyValue) \
@implementation NSArray (Conversions_##TYPE) \
+ (struct TYPE *)ffi_to:(NSArray *)obj { \
    struct TYPE *ffi_ref = malloc(sizeof(struct TYPE)); \
    ffi_ref->count = [obj count]; \
    NSUInteger i = 0; \
    for (id *key in obj) { \
        ffi_ref->values[i] = ToValue; \
        i++; \
    } \
    return ffi_ref; \
} \
+ (struct TYPE *)ffi_to_opt:(NSArray *_Nullable)obj { \
    return obj ? [self ffi_to:obj] : nil; \
} \
+ (NSArray *)ffi_from:(struct TYPE *)ffi_ref { \
    NSMutableArray *obj = [NSMutableArray arrayWithCapacity:ffi_ref->count]; \
    for (NSUInteger i = 0; i < ffi_ref->count; i++) { \
        [obj addObject:FromValue]; \
    } \
    return obj; \
} \
+ (NSArray *_Nullable)ffi_from_opt:(struct TYPE *)ffi_ref { \
    return ffi_ref ? [self ffi_from:ffi_ref] : nil; \
} \
+ (void)ffi_destroy:(struct TYPE *)ffi_ref { \
    if (!ffi_ref) return; \
    DestroyValue \
    free(ffi_ref); \
} \
@end \
\
@implementation NSArray (Bindings_##TYPE) \
+ (struct TYPE *)ffi_ctor:(NSArray *)obj { \
    NSUInteger i = 0, count = [obj count]; \
    VarValue *values = malloc(count * sizeof(VarValue)); \
    for (id *key in obj) { \
        values[i] = ToValue; \
        i++; \
    } \
    return ##TYPE_ctor(count, keys, values); \
} \
+ (void)ffi_dtor:(struct TYPE *)ffi_ref { \
    ##TYPE_destroy(ffi_ref); \
} \
@end
    // NSUInteger count = [obj count];
    // struct std_collections_Map_keys_dash_spv_masternode_processor_common_block_Block_values_dash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey *ffi_ref = malloc(sizeof(struct std_collections_Map_keys_dash_spv_masternode_processor_common_block_Block_values_dash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey));
    // dash_spv_masternode_processor_common_block_Block **keys = malloc(count * sizeof(struct dash_spv_masternode_processor_common_block_Block));
    // dash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey **values = malloc(count * sizeof(struct dash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey));
    // for (id key in obj) {
    //     keys[i] = [DSdash_spv_masternode_processor_common_block_Block ffi_to:key];
    //     values[i] = [DSdash_spv_masternode_processor_models_operator_public_key_OperatorPublicKey ffi_to:obj[key]];
    // }
    // ffi_ref->count = count;
    // ffi_ref->keys = keys;
    // ffi_ref->values = values;
    // return ffi_ref;

#define FFIMapConversion(TYPE, \
    KeyTypeC, KeyTypeObjC, KeyCtor, KeyDtor, KeyFrom, KeyTo, \
    ValueTypeC, ValueTypeObjC, ValueCtor, ValueDtor, ValueFrom, ValueTo) \
@implementation NSDictionary (Conversions_##TYPE) \
- (struct TYPE *)ffi_to:(NSDictionary *)obj { \
    NSUInteger i = 0, count = [obj count]; \
    struct TYPE *ffi_ref = malloc(sizeof(struct TYPE)); \
    KeyTypeC *keys = malloc(count * sizeof(KeyTypeC)); \
    ValueTypeC *values = malloc(count * sizeof(ValueTypeC)); \
    for (id key in obj) { \
        keys[i] = KeyTo; \
        values[i] = ValueTo; \
        i++; \
    } \
    ffi_ref->count = count; \
    ffi_ref->keys = keys; \
    ffi_ref->values = values; \
    return ffi_ref; \
} \
+ (struct TYPE *)ffi_to_opt:(NSDictionary * _Nullable)obj { \
    return obj ? [self ffi_to:obj] : nil; \
} \
- (NSDictionary *)ffi_from:(struct TYPE *)ffi_ref { \
    uintptr_t count = ffi_ref->count; \
    NSMutableDictionary *obj = [NSMutableDictionary dictionaryWithCapacity:count]; \
    for (int i = 0; i < count; i++) { \
        [obj setObject:ValueFrom forKey:KeyFrom]; \
    } \
    return obj; \
} \
+ (NSDictionary * _Nullable)ffi_from_opt:(struct TYPE *)ffi_ref { \
    return ffi_ref ? [self ffi_from:ffi_ref] : nil; \
} \
+ (void)ffi_destroy:(struct TYPE *)ffi_ref { \
    if (!ffi_ref) return; \
    if (ffi_ref->count > 0) { \
        for (int i = 0; i < ffi_ref->count; i++) { \
            KeyDtor\
            ValueDtor\
        } \
        free(ffi_ref->keys); \
        free(ffi_ref->values); \
    } \
    free(ffi_ref); \
} \
@end \
@implementation NSDictionary (Bindings_##TYPE) \
+ (struct TYPE *)ffi_ctor:(NSDictionary *)obj { \
    NSUInteger i = 0, count = [obj count]; \
    KeyTypeC *keys = malloc(count * sizeof(KeyTypeC)); \
    ValueTypeC *values = malloc(count * sizeof(ValueTypeC)); \
    for (id key in obj) { \
        keys[i] = KeyTo; \
        values[i] = ValueTo; \
        i++; \
    } \
    return ##TYPE_ctor(count, keys, values); \
} \
+ (void)ffi_dtor:(struct TYPE *)ffi_ref { \
    ##TYPE_destroy(ffi_ref); \
} \
@end
"#;

pub const INTERFACES: &str = r#"
#define AS_OBJC(context) ((__bridge DSFermentContext *)(context))
#define AS_RUST(context) ((__bridge void *)(context))

@implementation NSString (Ferment)
+ (NSString *)ffi_from:(char *)ffi_ref {
    if (ffi_ref == NULL) {
        return nil;
    } else {
        NSString *address = [NSString stringWithUTF8String:ffi_ref];
        str_destroy(ffi_ref);
        return address;
    }
}
+ (NSString * _Nullable)ffi_from_opt:(char *)ffi_ref {
    return ffi_ref ? [self ffi_from:ffi_ref] : nil;
}
+ (char *)ffi_to:(NSString *)obj {
    return [obj UTF8String];
}
+ (char *)ffi_to_opt:(NSString * _Nullable)obj {
    return obj ? [self ffi_to:obj] : nil;
}
+ (void)ffi_destroy:(char *)ffi_ref {
    free(ffi_ref);
}
@end

// FFIGroupConversion(int8_t, int8_t*, obj.intValue, @(key));
// FFIGroupConversion(uint8_t, uint8_t*, obj.unsignedIntValue, @(key));
// FFIGroupConversion(int16_t, int16_t*, obj.intValue, @(key));
// FFIGroupConversion(uint16_t, uint16_t*, obj.unsignedIntValue, @(key));
// FFIGroupConversion(int32_t, int32_t*, obj.intValue, @(key));
// FFIGroupConversion(uint32_t, uint32_t*, obj.longLongValue, @(key));
// FFIGroupConversion(int64_t, int64_t*, obj.unsignedIntValue, @(key));
// FFIGroupConversion(uint64_t, uint64_t*, obj.unsignedLongLongValue, @(key));
// FFIGroupConversion(BOOL, bool*, obj.boolValue, @(key));

"#;