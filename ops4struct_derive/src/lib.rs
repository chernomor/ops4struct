// vim: set expandtab ts=4 sw=4:
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// https://docs.rs/syn/
// https://docs.rs/quote/

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn;
use syn::spanned::Spanned;

#[proc_macro_derive(Add)]
pub fn add(input: TokenStream) -> TokenStream {
    let (struct_name, fields) = match prepare(input) {
        Ok(r) => r,
        Err(stream) => return stream,
    };

    let ops = fields.iter().map(|field| {
        let ident = match field.ident.as_ref(){
            Some(i) => i,
            None => return quote_spanned! {
                field.ty.span()=>compile_error!("You can only derive this on structs with named fields!");
            }.into(),
        };
        quote! { #ident: self.#ident + other.#ident }
    });

    let ops_ref_r = ops.clone();
    let ops_ref_l = ops.clone();
    let ops_ref_b = ops.clone();
    let tokens = quote! {
        impl Add for #struct_name {
            type Output = #struct_name;
            fn add(self, other: #struct_name) -> #struct_name {
                #struct_name {
                    #( #ops, )*
                }
            }
        }
        impl Add<&#struct_name> for #struct_name {
            type Output = <#struct_name as Add<#struct_name>>::Output;
            fn add(self, other: &#struct_name) -> <#struct_name as Add<#struct_name>>::Output {
                #struct_name {
                    #( #ops_ref_l, )*
                }
            }
        }
        impl<'a> Add<#struct_name> for &'a #struct_name {
            type Output = <#struct_name as Add<#struct_name>>::Output;
            fn add(self, other: #struct_name) -> <#struct_name as Add<#struct_name>>::Output {
                #struct_name {
                    #( #ops_ref_r, )*
                }
            }
        }

        impl Add<&#struct_name> for &#struct_name {
            type Output = <#struct_name as Add<#struct_name>>::Output;
            fn add(self, other: &#struct_name) -> <#struct_name as Add<#struct_name>>::Output {
                #struct_name {
                    #( #ops_ref_b, )*
                }
            }
        }
    };

    tokens.into()
}
#[proc_macro_derive(AddAssign)]
pub fn add_assign(input: TokenStream) -> TokenStream {
    let (struct_name, fields) = match prepare(input) {
        Ok(r) => r,
        Err(stream) => return stream,
    };

    let ops = fields.iter().map(|field| {
        let ident = match field.ident.as_ref(){
            Some(i) => i,
            None => return quote_spanned! {
                field.ty.span()=>compile_error!("You can only derive this on structs with named fields!");
            }.into(),
        };
        quote! { #ident: self.#ident + other.#ident }
    });

    let ops2 = ops.clone();
    let tokens = quote! {
        impl AddAssign for #struct_name {
            fn add_assign(&mut self, other: #struct_name) {
                *self = #struct_name {
                    #( #ops, )*
                }
            }
        }
        impl AddAssign<&#struct_name> for #struct_name {
            fn add_assign(&mut self, other: &#struct_name) {
                *self = #struct_name {
                    #( #ops2, )*
                }
            }
        }
    };

    tokens.into()
}

#[proc_macro_derive(Sub)]
pub fn sub(input: TokenStream) -> TokenStream {
    let (struct_name, fields) = match prepare(input) {
        Ok(r) => r,
        Err(stream) => return stream,
    };

    let ops = fields.iter().map(|field| {
        let ident = match field.ident.as_ref(){
            Some(i) => i,
            None => return quote_spanned! {
                field.ty.span()=>compile_error!("You can only derive this on structs with named fields!");
            }.into(),
        };
        quote! { #ident: self.#ident - other.#ident }
    });

    let ops_ref_r = ops.clone();
    let ops_ref_l = ops.clone();
    let ops_ref_b = ops.clone();
    let tokens = quote! {
        impl Sub for #struct_name {
            type Output = #struct_name;
            fn sub(self, other: #struct_name) -> #struct_name {
                #struct_name {
                    #( #ops, )*
                }
            }
        }
        impl Sub<&#struct_name> for #struct_name {
            type Output = <#struct_name as Sub<#struct_name>>::Output;
            fn sub(self, other: &#struct_name) -> <#struct_name as Sub<#struct_name>>::Output {
                #struct_name {
                    #( #ops_ref_l, )*
                }
            }
        }
        impl<'a> Sub<#struct_name> for &'a #struct_name {
            type Output = <#struct_name as Sub<#struct_name>>::Output;
            fn sub(self, other: #struct_name) -> <#struct_name as Sub<#struct_name>>::Output {
                #struct_name {
                    #( #ops_ref_r, )*
                }
            }
        }

        impl Sub<&#struct_name> for &#struct_name {
            type Output = <#struct_name as Sub<#struct_name>>::Output;
            fn sub(self, other: &#struct_name) -> <#struct_name as Sub<#struct_name>>::Output {
                #struct_name {
                    #( #ops_ref_b, )*
                }
            }
        }
    };

    tokens.into()
}
#[proc_macro_derive(SubAssign)]
pub fn sub_assign(input: TokenStream) -> TokenStream {
    let (struct_name, fields) = match prepare(input) {
        Ok(r) => r,
        Err(stream) => return stream,
    };

    let ops = fields.iter().map(|field| {
        let ident = match field.ident.as_ref(){
            Some(i) => i,
            None => return quote_spanned! {
                field.ty.span()=>compile_error!("You can only derive this on structs with named fields!");
            }.into(),
        };
        quote! { #ident: self.#ident - other.#ident }
    });

    let tokens = quote! {
        impl SubAssign for #struct_name {
            fn sub_assign(&mut self, other: #struct_name) {
                *self = #struct_name {
                    #( #ops, )*
                }
            }
        }
    };

    tokens.into()
}


fn prepare(input: TokenStream)
    -> Result<(proc_macro2::Ident, syn::punctuated::Punctuated<syn::Field, syn::token::Comma>), TokenStream>
{
    let ast: syn::DeriveInput = match syn::parse(input) {
        Ok(a) => a,
        Err(e) => return Err(e.to_compile_error().into()),
    };
    let struct_name = ast.ident.clone();

    match ast.data {
        syn::Data::Struct(datastruct) => match datastruct.fields {
            syn::Fields::Named(fields) => {
                Ok((struct_name, fields.named.clone()))
            },
            _ => Err(quote_spanned! {
                datastruct.struct_token.span => compile_error!("You can only derive this on structs with named fields!");
            }.into()),
        },
        _ => Err(quote_spanned! {
            ast.ident.span() => compile_error!("You can only derive this on structs with named fields!");
        }.into()),
    }
}

