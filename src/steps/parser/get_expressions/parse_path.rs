use std::path::{Path, PathBuf};
use crate::errors::soul_error::{new_soul_error, Result};
use crate::steps::step_interfaces::i_parser::scope::LookUpScope;
use crate::{errors::soul_error::{SoulError, SoulErrorKind}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{soul_type::type_kind::TypeKind, spanned::Spanned}, scope::{InnerScope, ScopeBuilder, SoulPagePath, TypeScopeStack}}, i_tokenizer::TokenStream}};

pub struct PagePathKind {
    pub path: SoulPagePath,
    pub is_page: bool,
}

pub fn get_page_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<PagePathKind>> {
    inner_get_page_path(stream, scopes, None)
}

pub fn get_page_path_type_stack(stream: &mut TokenStream, scopes: &mut ScopeBuilder, types: &TypeScopeStack) -> Result<Spanned<PagePathKind>> {
    inner_get_page_path(stream, scopes, Some(types))
}

fn inner_get_page_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder, possible_types: Option<&TypeScopeStack>) -> Result<Spanned<PagePathKind>> {
    let (types, current_index) = if let Some(tys) = possible_types {
        (&tys.scopes, tys.current)
    } 
    else {
        (scopes.get_types(), scopes.current_index())
    }; 

    let path_i = stream.current_index();
    let mut path = resolve_first_path(stream, scopes, types, current_index)?;

    loop {
        stream.next();
        if !stream.peek_multiple(2).is_some_and(|t| t.text == "::") {
            break;
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
        path.push(Path::new(stream.current_text()));
    }

    let mut page_path = SoulPagePath::from_path(&path);

    if is_book(&scopes, &page_path) {
        
        if stream.next().is_none() { 
            return Err(err_out_of_bounds(stream)); 
        }

        path.push(Path::new(stream.current_text()));
        page_path = SoulPagePath::from_path(&path);

        let result = if is_book(&scopes, &page_path) {
            PagePathKind { path: page_path, is_page: false }
        } 
        else if is_page(&scopes, &page_path) {
            PagePathKind { path: page_path, is_page: true }
        } 
        else {
            return Err(err_invalid_path(stream, path_i, &page_path.0));
        };

        return Ok(Spanned::new(result, stream[path_i].span.combine(&stream.current_span())));
    }

    if !is_page(&scopes, &page_path) {
        path.pop();
        stream.next_multiple(-2);
        let old = page_path;
        page_path = SoulPagePath::from_path(&path);

        if !is_page(&scopes, &page_path) {
            stream.next_multiple(2);
            return Err(err_invalid_path(stream, path_i, &old.0));
        }
    }

    scopes.external_pages.push(page_path.clone());

    Ok(Spanned::new(
        PagePathKind { path: page_path, is_page: true },
        stream[path_i].span.combine(&stream.current_span()),
    ))
}

fn resolve_first_path(
    stream: &mut TokenStream,
    scopes: &ScopeBuilder,
    types: &Vec<InnerScope<TypeKind>>,
    current_index: usize
) -> Result<PathBuf> {
    if stream.current_text() == "this" {
        return Ok(PathBuf::from(Path::new(&scopes.project_name)));
    }
    if let Some(TypeKind::ExternalPath { path, .. }) =
        types.lookup(&stream.current_text(), current_index)
    {
        let mut path = path.to_path_buf(false);
        if stream.peek_multiple(3).is_some_and(|t| t.text != "::") {
            stream.next_multiple(2);
            path.push(Path::new(stream.current_text()));
        }
        return Ok(path);
    }
    Ok(PathBuf::from(Path::new(stream.current_text())))
}

fn err_invalid_path(
    stream: &TokenStream,
    start: usize,
    path_str: &str
) -> SoulError {
    new_soul_error(
        SoulErrorKind::InvalidType,
        stream[start].span.combine(&stream.current_span()),
        format!("path '{}' not found", path_str),
    )
}

fn is_book(scopes: &ScopeBuilder, page_path: &SoulPagePath) -> bool {
    scopes.is_external_book(page_path).is_some()
}

fn is_page(scopes: &ScopeBuilder, page_path: &SoulPagePath) -> bool {
    scopes.is_external_page(page_path).is_some()
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing path")
}
























