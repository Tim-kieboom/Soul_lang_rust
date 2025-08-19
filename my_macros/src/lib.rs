use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

fn contains_multiref(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident == "MultiRef" {
                    return true;
                }
                if let syn::PathArguments::AngleBracketed(ref args) = last_segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(inner_ty) = arg {
                            if contains_multiref(inner_ty) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}

#[proc_macro_derive(CloneWithPool)]
pub fn derive_clone_with_pool(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {

                    let clone_fields = fields_named.named.iter().map(|f| {
                        let ident = f.ident.as_ref().expect("expected named field");
                        let ty = &f.ty;
                        if contains_multiref(ty) {
                            quote! {
                                #ident: self.#ident.clone_change_ref_pool(src_pool, dst_pool)
                            }
                        } else {
                            quote! {
                                #ident: self.#ident.clone()
                            }
                        }
                    });
                    quote! {
                        impl #impl_generics CloneWithPool for #name #ty_generics #where_clause {
                            fn clone_change_ref_pool(&self, src_pool: &MultiRefPool, dst_pool: &mut MultiRefPool) -> Self {
                                Self {
                                    #(#clone_fields),*
                                }
                            }
                        }
                    }
                },
                Fields::Unnamed(fields_unnamed) => {

                    let clone_fields = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| {
                        let ty = &f.ty;
                        let index = syn::Index::from(i);
                        if contains_multiref(ty) {
                            quote! {
                                self.#index.clone_change_ref_pool(src_pool, dst_pool)
                            }
                        } else {
                            quote! {
                                self.#index.clone()
                            }
                        }
                    });
                    quote! {
                        impl #impl_generics CloneWithPool for #name #ty_generics #where_clause {
                            fn clone_change_ref_pool(&self, src_pool: &MultiRefPool, dst_pool: &mut MultiRefPool) -> Self {
                                Self(
                                    #(#clone_fields),*
                                )
                            }
                        }
                    }
                },
                Fields::Unit => {

                    quote! {
                        impl #impl_generics CloneWithPool for #name #ty_generics #where_clause {
                            fn clone_change_ref_pool(&self, _src_pool: &MultiRefPool, _dst_pool: &mut MultiRefPool) -> Self {
                                *self
                            }
                        }
                    }
                }
            }
        },
        Data::Enum(data_enum) => {
            let variant_arms = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields_named) => {
                        let field_idents: Vec<_> = fields_named.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                        let clone_fields = fields_named.named.iter().map(|f| {
                            let ident = f.ident.as_ref().unwrap();
                            let ty = &f.ty;
                            if contains_multiref(ty) {
                                quote! { #ident: #ident.clone_change_ref_pool(src_pool, dst_pool) }
                            } else {
                                quote! { #ident: #ident.clone() }
                            }
                        });
                        quote! {
                            Self::#variant_name { #(ref #field_idents),* } => {
                                Self::#variant_name {
                                    #(#clone_fields),*
                                }
                            }
                        }
                    },
                    Fields::Unnamed(fields_unnamed) => {
                        let field_idents: Vec<_> = (0..fields_unnamed.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site()))
                            .collect();
                        let clone_fields = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| {
                            let ty = &f.ty;
                            let ident = &field_idents[i];
                            if contains_multiref(ty) {
                                quote! { #ident.clone_change_ref_pool(src_pool, dst_pool) }
                            } else {
                                quote! { #ident.clone() }
                            }
                        });
                        quote! {
                            Self::#variant_name ( #(ref #field_idents),* ) => {
                                Self::#variant_name (
                                    #(#clone_fields),*
                                )
                            }
                        }
                    },
                    Fields::Unit => {
                        quote! {
                            Self::#variant_name => Self::#variant_name
                        }
                    }
                }
            });

            quote! {
                impl #impl_generics CloneWithPool for #name #ty_generics #where_clause {
                    fn clone_change_ref_pool(&self, src_pool: &MultiRefPool, dst_pool: &mut MultiRefPool) -> Self {
                        match *self {
                            #(#variant_arms),*
                        }
                    }
                }
            }
        },
        _ => panic!("#[derive(CloneWithPool)] is only supported on structs and enums"),
    };

    TokenStream::from(expanded)
}



















