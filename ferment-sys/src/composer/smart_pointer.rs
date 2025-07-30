use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Type};
use ferment_macro::ComposerBase;
use crate::ast::Depunctuated;
use crate::composable::{AttrsModel, FieldTypeKind, GenModel, LifetimesModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerLink, BasicComposerOwner, ComposerLink, GenericComposerInfo, NameKind, SourceComposable};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::SmartPointerKind;
use crate::ext::{GenericNestedArg, LifetimeProcessor, Mangle, ToType};
use crate::lang::{FromDictionary, RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, SmartPointerPresentableContext, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, DocComposer, InterfacePresentation, InterfacesMethodExpr};

#[derive(ComposerBase)]
pub struct SmartPointerComposer<SPEC>
where SPEC: Specification + 'static {
    pub ty: Type,
    kind: SmartPointerKind,
    base: BasicComposerLink<SPEC, Self>,
}

impl<SPEC> SmartPointerComposer<SPEC>
where SPEC: Specification {
    pub fn new(ty: &Type, kind: SmartPointerKind, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(DocComposer::new(ty_context.to_token_stream()), AttrsModel::from(&attrs), ty_context, GenModel::default(), LifetimesModel::default(), Rc::clone(scope_context)),
            ty: ty.clone(),
            kind
        }
    }
}

impl SourceComposable for SmartPointerComposer<RustSpecification> {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustSpecification>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {

        let arg_ty = self.ty.maybe_first_nested_type_ref()?;
        let wrap_alloc = |expr| expr;
        let wrap_from = |expr| DictionaryExpr::from_ptr_read(expr);
        // let wrap_to_arg = |expr| Expression::Clone(Box::from(expr));

        let ffi_name = self.ty.mangle_tokens_default();
        let lifetimes = arg_ty.unique_lifetimes();
        let types = (self.present_ffi_aspect(), self.present_target_aspect());
        let attrs = self.compose_attributes();

        let arg_0_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Obj);
        let value_name = <RustSpecification as Specification>::Name::dictionary_name(DictionaryName::Value);

        let from_body = DictionaryExpr::from_root(wrap_from(DictionaryExpr::ffi_ref_prop(&arg_0_name)));
        let to_body = InterfacesMethodExpr::Boxed(DictionaryExpr::self_destruct(arg_0_name.field_composer(FieldTypeKind::conversion(InterfacesMethodExpr::Boxed(arg_0_name.to_token_stream()))).present(source)).to_token_stream());
        let drop_body = InterfacesMethodExpr::UnboxAny(DictionaryExpr::self_prop(&arg_0_name).to_token_stream());

        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::non_generic_conversion_from(&attrs, &types, from_body, &lifetimes),
            InterfacePresentation::non_generic_conversion_to(&attrs, &types, to_body, &lifetimes),
            InterfacePresentation::drop(&attrs, ffi_name.to_type(), drop_body)
        ]);

        let aspect = Aspect::raw_struct_ident(self.ty.mangle_ident_default());

        let root_var = <RustSpecification as Specification>::value_var(&self.ty).compose(source);
        let ctor_arg_var = <RustSpecification as Specification>::value_var(&arg_ty).compose(source);
        let ctor_arg_type = ctor_arg_var.to_type();

        let root_field_type_kind = FieldTypeKind::Var(root_var);
        let arg_field_type_kind = FieldTypeKind::Var(ctor_arg_var.clone());
        let raw_field_type_kind = FieldTypeKind::Var(<RustSpecification as Specification>::Var::mut_ptr(self.ty.clone()));

        let root_arg_expr = <RustSpecification as Specification>::Expr::name(&arg_0_name);
        let value_arg_expr = <RustSpecification as Specification>::Expr::name(&value_name);


        let root_field_composer = arg_0_name.field_composer(raw_field_type_kind);
        let arg_field_composer = value_name.field_composer(arg_field_type_kind.clone());
        let root_arg_composer = arg_0_name.field_composer(root_field_type_kind);
        let ctor_arg_composer = arg_0_name.field_composer(arg_field_type_kind);

        let from_arg_conversion = <RustSpecification as Specification>::value_expr_from(arg_0_name.clone(), &arg_ty, root_arg_expr.clone())
            .compose(source);
        let from_root_obj_conversion = <RustSpecification as Specification>::value_expr_from(arg_0_name.clone(), &self.ty, root_arg_expr.clone())
            .compose(source);
        let from_arg_value_conversion = <RustSpecification as Specification>::value_expr_from(arg_0_name.clone(), &arg_ty, value_arg_expr)
            .compose(source);
        let to_arg_conversion = <RustSpecification as Specification>::value_expr_to(arg_0_name.clone(), &arg_ty, self.kind.wrap_arg_to(root_arg_expr))
            .compose(source);
        let ctor_to_arg_expr = wrap_alloc(Expression::new_smth(self.kind.is_once_lock().then(|| Expression::Empty).unwrap_or_else(|| from_arg_conversion), self.kind.dictionary_type()));

        let bindings = Depunctuated::from_iter([
            self.kind.binding_presentable(&aspect, &attrs, &lifetimes, SmartPointerPresentableContext::Ctor(ctor_arg_composer, ctor_to_arg_expr)),
            self.kind.binding_presentable(&aspect, &attrs, &lifetimes, SmartPointerPresentableContext::Dtor(<RustSpecification as Specification>::Gen::default(), NameKind::Named)),
            self.kind.binding_presentable(&aspect, &attrs, &lifetimes, SmartPointerPresentableContext::Read(root_arg_composer.clone(), ctor_arg_type, from_root_obj_conversion.clone(), to_arg_conversion)),
            self.kind.binding_presentable(&aspect, &attrs, &lifetimes, SmartPointerPresentableContext::Write(root_arg_composer, arg_field_composer, from_root_obj_conversion, from_arg_value_conversion))
        ]);
        Some(GenericComposerInfo::<RustSpecification>::default_with_bindings(
            aspect,
            &attrs,
            Depunctuated::from_iter([root_field_composer]),
            interfaces,
            bindings
        ))
    }
}

