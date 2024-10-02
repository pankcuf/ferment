use crate::lang::objc::presentable::TypeContext;
use crate::presentable::Aspect;

pub struct ClassNameComposer {
    pub aspect: Aspect<TypeContext>
}

// impl<'a> Composer for ClassNameComposer {
//     type Source = (&str, &ScopeContext);
//     type Output = Name;
//
//     fn compose(&self, (class_prefix, source): &Self::Source) -> Self::Output {
//         Name::Ident(format_ident!("{}{}", class_prefix, self.aspect.present(source).to_token_stream().to_string()))
//     }
// }
