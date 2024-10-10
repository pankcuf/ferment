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

#define FFIMapConversion(TYPE, VarKey, FromKey, ToKey, DestroyKey, VarValue, FromValue, ToValue, DestroyValue) \
@implementation NSDictionary (Conversions_##TYPE) \
- (struct TYPE *)ffi_to:(NSDictionary *)obj { \
    struct TYPE *ffi_ref = malloc(sizeof(struct TYPE)); \
    ffi_ref->count = [obj count]; \
    NSUInteger i = 0; \
    for (id *key in obj) { \
        ffi_ref->keys[i] = ToKey; \
        ffi_ref->values[i] = ToValue; \
        i++; \
    } \
    return ffi_ref; \
} \
+ (struct TYPE *)ffi_to_opt:(NSDictionary * _Nullable)obj { \
    return obj ? [self ffi_to:obj] : nil; \
} \
- (NSDictionary *)ffi_from:(struct TYPE *)ffi_ref { \
    NSMutableDictionary *obj = [NSMutableDictionary dictionaryWithCapacity:ffi_ref->count]; \
    for (NSUInteger i = 0; i < ffi_ref->count; i++) { \
        [obj setObject:FromKey forKey:FromValue]; \
    } \
    return obj; \
} \
+ (NSDictionary * _Nullable)ffi_from_opt:(struct TYPE *)ffi_ref { \
    return ffi_ref ? [self ffi_from:ffi_ref] : nil; \
} \
+ (void)ffi_destroy:(struct TYPE *)ffi_ref { \
    if (!ffi_ref) return; \
    DestroyKey \
    DestroyValue \
    free(ffi_ref); \
} \
@end \
@implementation NSDictionary (Bindings_##TYPE) \
+ (struct TYPE *)ffi_ctor:(NSDictionary *)obj { \
    NSUInteger i = 0, count = [obj count]; \
    VarKey *keys = malloc(count * sizeof(VarKey)); \
    VarValue *values = malloc(count * sizeof(VarValue)); \
    for (id *key in obj) { \
        keys[i] = ToKey; \
        values[i] = ToValue; \
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