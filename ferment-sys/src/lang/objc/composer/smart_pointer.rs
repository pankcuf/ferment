use quote::ToTokens;
use crate::ast::Depunctuated;
use crate::composer::{AttrComposable, GenericComposerInfo, SmartPointerComposer, SourceComposable, TypeAspect};
#[cfg(feature = "accessors")]
use crate::composer::NameKind;
use crate::context::ScopeContext;
use crate::ext::{AsType, PunctuateOne};
#[cfg(feature = "accessors")]
use crate::ext::{LifetimeProcessor, ToType};
use crate::kind::FieldTypeKind;
use crate::lang::Specification;
use crate::lang::objc::fermentate::InterfaceImplementation;
use crate::lang::objc::ObjCSpecification;
#[cfg(feature = "accessors")]
use crate::presentable::{Expression, SmartPointerPresentableContext};

impl SourceComposable for SmartPointerComposer<ObjCSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<ObjCSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {

        let root_ty_ref = self.root_kind.as_type();
        let attrs = self.compose_attributes();

        let arg_0_name = <ObjCSpecification as Specification>::Name::obj();
        let interfaces = Depunctuated::<InterfaceImplementation>::new();
        let aspect = self.raw_target_type_aspect();
        let raw_field_type_kind = FieldTypeKind::Var(<ObjCSpecification as Specification>::Var::mut_ptr(root_ty_ref.to_token_stream()));
        let root_field_composer = arg_0_name.field_composer(raw_field_type_kind);


        let bindings = {
            #[cfg(feature = "accessors")]
            {
                let arg_ty = self.root_kind.wrapped_arg_type()?;
                let lifetimes = arg_ty.unique_lifetimes();
                let value_name = <ObjCSpecification as Specification>::Name::value();
                let ctor_arg_var = <ObjCSpecification as Specification>::value_var(arg_ty).compose(source);
                let root_var = <ObjCSpecification as Specification>::value_var(root_ty_ref).compose(source);
                let ctor_arg_type = ctor_arg_var.to_type();
                let root_field_type_kind = FieldTypeKind::Var(root_var);
                let arg_field_type_kind = FieldTypeKind::Var(ctor_arg_var.clone());
                let root_arg_expr = <ObjCSpecification as Specification>::Expr::name(&arg_0_name);
                let value_arg_expr = <ObjCSpecification as Specification>::Expr::name(&value_name);
                let arg_field_composer = value_name.field_composer(arg_field_type_kind.clone());
                let root_arg_composer = arg_0_name.field_composer(root_field_type_kind);
                let ctor_arg_composer = arg_0_name.field_composer(arg_field_type_kind);
                let from_arg_conversion = <ObjCSpecification as Specification>::value_ref_expr_from(&arg_0_name, arg_ty, root_arg_expr.clone())
                    .compose(source);
                let from_root_obj_conversion = <ObjCSpecification as Specification>::value_ref_expr_from(&arg_0_name, root_ty_ref, root_arg_expr.clone())
                    .compose(source);
                let from_arg_value_conversion = <ObjCSpecification as Specification>::value_ref_expr_from(&arg_0_name, arg_ty, value_arg_expr)
                    .compose(source);
                let to_arg_conversion = <ObjCSpecification as Specification>::value_ref_expr_to(&arg_0_name, arg_ty, self.kind.wrap_arg_to(root_arg_expr))
                    .compose(source);
                let ctor_to_arg_expr = self.root_kind.wrap_alloc::<ObjCSpecification>(
                    Expression::new_smth(
                        if self.kind.is_once_lock() { Expression::Empty } else { from_arg_conversion },
                        self.kind.dictionary_type()));
                let signature_aspect = (attrs.clone(), lifetimes, Default::default());
                Depunctuated::from_iter([
                    self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Ctor(ctor_arg_composer, ctor_to_arg_expr)),
                    self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Dtor(NameKind::Named)),
                    self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Read(root_arg_composer.clone(), ctor_arg_type, from_root_obj_conversion.clone(), to_arg_conversion)),
                    self.kind.binding_presentable(&aspect, &signature_aspect, SmartPointerPresentableContext::Write(root_arg_composer, arg_field_composer, from_root_obj_conversion, from_arg_value_conversion))
                ])
            }
            #[cfg(not(feature = "accessors"))]
            Default::default()
        };

        Some(GenericComposerInfo::<ObjCSpecification>::default_with_bindings(
            aspect,
            &attrs,
            root_field_composer.punctuate_one(),
            interfaces,
            bindings
        ))
    }
}
