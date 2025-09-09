use proc_macro2::Ident;
use syn::{Path, PathSegment};
use crate::ast::Colon2Punctuated;
use crate::context::{GlobalContext, ScopeChain};
use crate::ext::{Join, PathTransform, Pop, ToPath, ToPathSepSegments, CRATE, SELF, SUPER};

pub enum ReexportSeek {
    Absolute,
    Relative,
}

impl ReexportSeek {
    fn join_reexport(&self, import_path: &Path, scope_path: &Path, crate_name: &Ident, chunk: Option<&Path>) -> Path {
        // TODO: deal with "super::super::"
        match self {
            ReexportSeek::Absolute => if let Some(PathSegment { ident, .. }) = import_path.segments.first() {
                match (ident.to_string().as_str(), chunk) {
                    (CRATE, Some(chunk_ref)) =>
                        merge_reexport_chunks(Colon2Punctuated::from_iter(import_path.replaced_first_with(&crate_name.to_path())
                            .segments
                            .into_iter()
                            .skip(scope_path.segments.len()))
                                                  .to_path(), chunk_ref),
                    (CRATE, None) =>
                        Colon2Punctuated::from_iter(import_path.segments
                            .replaced_first_with(&crate_name.to_segments())
                            .iter()
                            .skip(scope_path.segments.len())
                            .cloned())
                            .to_path(),
                    (SELF, _) =>
                        Colon2Punctuated::from_iter(import_path.segments
                            .iter()
                            .skip(1)
                            .cloned())
                            .to_path(),
                    (SUPER, _) =>
                        scope_path.popped()
                            .joined(import_path),
                    (_, Some(chunk_ref)) =>
                        import_path.popped()
                            .joined(chunk_ref),
                    (_, None) =>
                        import_path.clone()
                }
            } else {
                import_path.clone()
            }
            ReexportSeek::Relative => if let Some(chunk) = chunk {
                import_path.popped().joined(chunk)
            } else {
                import_path.clone()
            }
        }
    }
    pub(crate) fn maybe_reexport(&self, path: &Path, source: &GlobalContext) -> Option<Path> {
        // println!("... maybe_reexport: {}", format_token_stream(path));
        let mut candidate = path.clone();
        let mut result: Option<Path> = None;
        let mut chunk: Option<Path> = None;
        while let Some(last_segment) = candidate.segments.last().cloned() {
            candidate = candidate.popped();
            // println!("... reexport candidate: {} --- {}", format_token_stream(&last_segment), format_token_stream(&candidate));
            match source.maybe_import_scope_pair_ref(&last_segment, &candidate) {
                Some((scope, import)) => {
                    let scope_path = scope.self_path_ref();
                    let reexport_path = self.join_reexport(import, scope_path, scope.crate_ident_ref(), chunk.as_ref());
                    result = Some(scope_path.joined(&reexport_path));
                    chunk = Some(reexport_path);
                }
                None => if candidate.segments.is_empty() {
                    return result;
                } else if let Some(reexport) = self.maybe_reexport(&candidate, source) {
                    result = Some(reexport);
                }
            }
        }
        result
    }
}

// Try to find the scope where item is actually defined
// assuming that 'path' is defined at 'scope' and can be shortened
#[allow(unused)]
pub(crate) fn maybe_closest_known_scope_for_import_in_scope<'a>(path: &'a Path, scope: &'a ScopeChain, source: &'a GlobalContext) -> Option<&'a ScopeChain> {
    // First assumption that it is relative import path
    let scope_path = scope.self_path_ref();
    let mut closest_scope: Option<&ScopeChain> = None;

    let mut chunk = path.popped();
    while !chunk.segments.is_empty() {
        let candidate = scope_path.joined(&chunk);
        closest_scope = source.maybe_scope_ref(&candidate);
        if closest_scope.is_some() {
            return closest_scope;
        }
        chunk = chunk.popped();
    }
    chunk = path.popped();
    // Second assumption that it is global import path;
    while !chunk.segments.is_empty() {
        closest_scope = source.maybe_scope_ref(&chunk);
        if closest_scope.is_some() {
            return closest_scope;
        }
        chunk = chunk.popped();
    }
    None
}

fn merge_reexport_chunks(mut base: Path, extension: &Path) -> Path {
    let mut base_segments: Vec<_> = base.segments.iter().collect();
    let mut ext_segments: Vec<_> = extension.segments.iter().collect();
    base_segments.reverse();
    ext_segments.reverse();
    let mut result_segments = vec![];
    let mut skip = 0;
    for (base_segment, ext_segment) in base_segments.iter().zip(ext_segments.iter()) {
        if base_segment.ident == ext_segment.ident {
            skip += 1;
        } else {
            break;
        }
    }
    base_segments.reverse();
    ext_segments.reverse();
    let base_len = base_segments.len();
    result_segments.extend(base_segments.into_iter().take(base_len - skip));
    result_segments.extend(ext_segments);
    base.segments = result_segments.into_iter().cloned().collect();
    base
}

