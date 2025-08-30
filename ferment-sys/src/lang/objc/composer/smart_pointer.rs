use quote::ToTokens;
use crate::ast::Depunctuated;
use crate::composer::{AttrComposable, GenericComposerInfo, NameKind, SmartPointerComposer, SourceComposable, TypeAspect};
use crate::context::ScopeContext;
use crate::ext::{AsType, LifetimeProcessor, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::{FromDictionary, Specification};
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::ObjCSpecification;
use crate::presentable::{Expression, SmartPointerPresentableContext};
use crate::presentation::{DictionaryExpr, DictionaryName};

impl SourceComposable for SmartPointerComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {

        let root_ty_ref = self.root_kind.as_type();
        let arg_ty = self.root_kind.wrapped_arg_type()?;

        // let ffi_name = root_ty_ref.mangle_tokens_default();
        let lifetimes = arg_ty.unique_lifetimes();
        // let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let attrs = self.compose_attributes();
        // let attrs = ;

        let arg_0_name = <ObjCSpecification as Specification>::Name::dictionary_name(DictionaryName::Obj);
        let value_name = <ObjCSpecification as Specification>::Name::dictionary_name(DictionaryName::Value);

        // let from_body = Expression::<ObjCSpecification>::dict_expr(DictionaryExpr::from_root(self.root_kind.wrap_from::<ObjCSpecification, DictionaryExpr>(DictionaryExpr::ffi_ref_prop(&arg_0_name)).present(source)));
        // let to_body = Expression::<ObjCSpecification>::interface_expr(InterfacesMethodExpr::Boxed(DictionaryExpr::self_destruct(arg_0_name.field_composer(FieldTypeKind::kind(InterfacesMethodExpr::Boxed(arg_0_name.to_token_stream()))).present(source)).to_token_stream()));
        // let drop_body = Expression::<ObjCSpecification>::interface_expr(InterfacesMethodExpr::UnboxAny(DictionaryExpr::self_prop(&arg_0_name).to_token_stream()));

        let interfaces = Depunctuated::<InterfaceImplementation>::new();
        // let interfaces = Depunctuated::<InterfaceImplementation>::from_iter([
        //     // InterfacePresentation::non_generic_conversion_from(&attrs.attrs, &types, from_body.present(source), &lifetimes),
        //     // InterfacePresentation::non_generic_conversion_to(&attrs.attrs, &types, to_body.present(source), &lifetimes),
        //     // InterfacePresentation::drop(&attrs.attrs, ffi_name.to_type(), drop_body.present(source))
        // ]);

        let aspect = self.raw_target_type_aspect();

        let root_var = <ObjCSpecification as Specification>::value_var(root_ty_ref).compose(source);
        let ctor_arg_var = <ObjCSpecification as Specification>::value_var(&arg_ty).compose(source);
        let ctor_arg_type = ctor_arg_var.to_type();

        let root_field_type_kind = FieldTypeKind::Var(root_var);
        let arg_field_type_kind = FieldTypeKind::Var(ctor_arg_var.clone());
        let raw_field_type_kind = FieldTypeKind::Var(<ObjCSpecification as Specification>::Var::mut_ptr(root_ty_ref.to_token_stream()));

        let root_arg_expr = <ObjCSpecification as Specification>::Expr::name(&arg_0_name);
        let value_arg_expr = <ObjCSpecification as Specification>::Expr::name(&value_name);

        let root_field_composer = arg_0_name.field_composer(raw_field_type_kind);
        let arg_field_composer = value_name.field_composer(arg_field_type_kind.clone());
        let root_arg_composer = arg_0_name.field_composer(root_field_type_kind);
        let ctor_arg_composer = arg_0_name.field_composer(arg_field_type_kind);

        let from_arg_conversion = <ObjCSpecification as Specification>::value_expr_from(arg_0_name.clone(), &arg_ty, root_arg_expr.clone())
            .compose(source);
        let from_root_obj_conversion = <ObjCSpecification as Specification>::value_expr_from(arg_0_name.clone(), root_ty_ref, root_arg_expr.clone())
            .compose(source);
        let from_arg_value_conversion = <ObjCSpecification as Specification>::value_expr_from(arg_0_name.clone(), &arg_ty, value_arg_expr)
            .compose(source);
        let to_arg_conversion = <ObjCSpecification as Specification>::value_expr_to(arg_0_name.clone(), &arg_ty, self.kind.wrap_arg_to(root_arg_expr))
            .compose(source);
        let ctor_to_arg_expr = self.root_kind.wrap_alloc::<ObjCSpecification, DictionaryExpr>(
            Expression::new_smth(
                self.kind.is_once_lock()
                    .then(|| Expression::Empty)
                    .unwrap_or_else(|| from_arg_conversion),
                self.kind.dictionary_type()));
        let generics = <ObjCSpecification as Specification>::Gen::default();
        let signature_aspect = (attrs, lifetimes, generics);
        let bindings = Depunctuated::from_iter([
            self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Ctor(ctor_arg_composer, ctor_to_arg_expr)),
            self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Dtor(NameKind::Named)),
            self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Read(root_arg_composer.clone(), ctor_arg_type, from_root_obj_conversion.clone(), to_arg_conversion)),
            self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Write(root_arg_composer, arg_field_composer, from_root_obj_conversion, from_arg_value_conversion))
        ]);
        Some(GenericComposerInfo::<ObjCSpecification>::default_with_bindings(
            aspect,
            &signature_aspect.0,
            Depunctuated::from_iter([root_field_composer]),
            interfaces,
            bindings
        ))
    }
}
