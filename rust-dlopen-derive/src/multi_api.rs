use super::common::get_fields;
use syn::{DeriveInput, Field};

const TRATIT_NAME: &str = "WrapperMultiApi";

pub fn impl_wrapper_multi_api(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let fields = get_fields(ast, TRATIT_NAME);

    let tok_iter = fields.named.iter().map(field_to_tokens);
    let q = quote! {
        impl #generics WrapperMultiApi for #name #generics{}

         impl #generics ::dlopen::wrapper::WrapperApi for # name #generics{
            unsafe fn load(lib: & ::dlopen::raw::Library) -> ::std::result::Result<Self,::dlopen::Error> {
                ::std::result::Result::Ok(#name {
                #(#tok_iter),*
                })
            }

        }
    };

    q
}

fn field_to_tokens(field: &Field) -> proc_macro2::TokenStream {
    let field_name = &field.ident;

    quote! {
        #field_name: ::dlopen::wrapper::WrapperApi::load(&lib)?
    }
}
