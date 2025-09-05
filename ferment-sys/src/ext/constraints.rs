use proc_macro2::Ident;
use syn::{AngleBracketedGenericArguments, AssocConst, AssocType, BareFnArg, Constraint, Expr, GenericArgument, ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf, ReturnType, Stmt, Type, TypeArray, TypeBareFn, TypeImplTrait, TypeParamBound, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple};
use syn::punctuated::Punctuated;

pub trait Constraints {
    fn has_self(&self) -> bool;
    fn has_no_self(&self) -> bool { !self.has_self() }
}

impl<T, P> Constraints for Punctuated<T, P> where T: Constraints {
    fn has_self(&self) -> bool {
        self.iter().find(|p| p.has_self()).is_some()
    }
}

impl Constraints for Constraint {
    fn has_self(&self) -> bool {
        self.ident.has_self()
    }
}

impl Constraints for Ident {
    fn has_self(&self) -> bool {
        self == "Self"
    }
}

impl Constraints for QSelf {
    fn has_self(&self) -> bool {
        self.ty.has_self()
    }
}

impl Constraints for TypeParamBound {
    fn has_self(&self) -> bool {
        if let TypeParamBound::Trait(bound) = self {
            bound.path.has_self()
        } else {
            false
        }
    }
}

impl Constraints for Path {
    fn has_self(&self) -> bool {
        self.segments.has_self()
    }
}

impl Constraints for GenericArgument {
    fn has_self(&self) -> bool {
        match self {
            GenericArgument::Lifetime(_) => false,
            GenericArgument::Type(ty) => ty.has_self(),
            GenericArgument::Const(_expr) => false, // TODO: Implement this
            // GenericArgument::Binding(binding) => binding.has_self(),
            GenericArgument::Constraint(constraint) => constraint.has_self(),
            GenericArgument::AssocType(assoc_type) => assoc_type.has_self(),
            GenericArgument::AssocConst(assoc_const) => assoc_const.has_self(),
            _ => false
        }
    }
}

impl Constraints for AssocType {
    fn has_self(&self) -> bool {
        self.ident.eq("Self") || self.ty.has_self() || self.generics.as_ref().map(|generics| generics.has_self()).unwrap_or_default()
    }
}
impl Constraints for AssocConst {
    fn has_self(&self) -> bool {
        self.ident.eq("Self") || self.value.has_self() || self.generics.as_ref().map(|generics| generics.has_self()).unwrap_or_default()
    }
}

impl Constraints for Stmt {
    fn has_self(&self) -> bool {
        match self {
            Stmt::Local(local) => local.init.as_ref().map(|aa| aa.expr.has_self() || aa.diverge.as_ref().map(|(_, e)| e.has_self()).unwrap_or_default()).unwrap_or_default(),
            Stmt::Item(_) => false,
            Stmt::Expr(expr, _) => expr.has_self(),
            Stmt::Macro(stmt_macro) => stmt_macro.mac.path.has_self()
        }
    }
}

impl Constraints for Expr {
    fn has_self(&self) -> bool {
        match self {
            Expr::Array(expr_array) => expr_array.elems.has_self(),
            Expr::Assign(expr_assign) => expr_assign.left.has_self() || expr_assign.right.has_self(),
            Expr::Binary(expr_binary) => expr_binary.left.has_self() || expr_binary.right.has_self(),
            Expr::Block(expr_block) => expr_block.block.stmts.iter().any(Constraints::has_self),
            // Expr::Call(expr_call) => expr_call.attrs.iter().any(Constraints::has_self) || expr_call.func.has_self(),
            // Expr::Cast(expr_cast) => expr_cast.expr.has_self() || expr_cast.ty.has_self(),
            // Expr::Closure(expr_closure) => expr_closure.inputs.iter().any(Constraints::has_self) || expr_closure.output.has_self() || expr_closure.body.has_self(),
            // Expr::Const(expr_const) => expr_const.block.stmts.iter().any(Constraints::has_self),
            // Expr::Continue(..) => false,
            // Expr::Field(expr_field) => expr_field.base.has_self(),
            // Expr::ForLoop(expr_for_loop) => expr_for_loop.expr.has_self() || expr_for_loop.body.stmts.iter().any(Constraints::has_self),
            // Expr::Group(expr_group) => expr_group.expr.has_self(),
            // Expr::If(expr_if) => expr_if.cond.has_self(),
            _ => false
            // Expr::Index(_) => {}
            // Expr::Infer(_) => {}
            // Expr::Let(_) => {}
            // Expr::Lit(_) => {}
            // Expr::Loop(_) => {}
            // Expr::Macro(_) => {}
            // Expr::Match(_) => {}
            // Expr::MethodCall(_) => {}
            // Expr::Paren(_) => {}
            // Expr::Path(_) => {}
            // Expr::Range(_) => {}
            // Expr::RawAddr(_) => {}
            // Expr::Reference(_) => {}
            // Expr::Repeat(_) => {}
            // Expr::Return(_) => {}
            // Expr::Struct(_) => {}
            // Expr::Try(_) => {}
            // Expr::TryBlock(_) => {}
            // Expr::Tuple(_) => {}
            // Expr::Unary(_) => {}
            // Expr::Unsafe(_) => {}
            // Expr::Verbatim(_) => {}
            // Expr::While(_) => {}
            // Expr::Yield(_) => {}
        }
    }
}
impl Constraints for AngleBracketedGenericArguments {
    fn has_self(&self) -> bool {
        todo!()
    }
}

impl Constraints for ReturnType {
    fn has_self(&self) -> bool {
        if let ReturnType::Type(_, ty) = self {
            ty.has_self()
        } else {
            false
        }
    }
}

impl Constraints for PathSegment {
    fn has_self(&self) -> bool {
        self.ident.has_self() || self.arguments.has_self()
    }
}

impl Constraints for PathArguments {
    fn has_self(&self) -> bool {
        match self {
            PathArguments::None => false,
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =>
                args.has_self(),
            PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) =>
                inputs.has_self() || output.has_self()
        }
    }
}

impl Constraints for Type {
    fn has_self(&self) -> bool {
        match self {
            Type::Array(TypeArray { elem, .. }) |
            Type::Paren(TypeParen { elem, .. }) |
            Type::Ptr(TypePtr { elem, .. }) |
            Type::Reference(TypeReference { elem, .. }) |
            Type::Slice(TypeSlice { elem, .. }) => elem.has_self(),
            Type::BareFn(TypeBareFn { inputs, output, .. }) =>
                inputs.iter().find(|BareFnArg { ty, .. }| ty.has_self()).is_some() || output.has_self(),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) |
            Type::TraitObject(TypeTraitObject { bounds, .. }) => bounds.has_self(),
            Type::Path(TypePath { qself, path }) => path.has_self() || qself.as_ref().map(Constraints::has_self).unwrap_or_default(),
            Type::Tuple(TypeTuple { elems, .. }) => elems.has_self(),
            _ => false,
        }
    }
}


