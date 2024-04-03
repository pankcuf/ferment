use quote::ToTokens;
use syn::Path;
use syn::punctuated::Punctuated;

pub trait Prefix {
    fn is_prefix_of(&self, other: &Self) -> bool;
    fn replace_prefix_with(&mut self, prefix: &Path, replacement: Path);
}

impl Prefix for Path {
    fn is_prefix_of(&self, path: &Self) -> bool {
        if self.segments.len() > path.segments.len() {
            return false;
        }
        let test = path.segments.last().unwrap().ident.to_string().eq("LLMQSnapshot");
        if test{
        println!("is_prefix_of {} ---- {}", self.to_token_stream(), path.to_token_stream());

        }
        self.segments.iter()
            .zip(path.segments.iter())
            .all(|(seg1, seg2)|
                seg1.ident == seg2.ident)
    }

    fn replace_prefix_with(&mut self, prefix: &Path, replacement: Path) {
        let mut new_segments = Punctuated::from_iter(replacement.segments.into_iter());
        new_segments.extend(self.segments.iter().skip(prefix.segments.len()).cloned());
        self.segments = new_segments;
    }
}