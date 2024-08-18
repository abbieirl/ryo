use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, Error, Fields};
use syn::spanned::Spanned;
use quote::quote;

pub(crate) fn derive_struct_input(input: DeriveInput) -> TokenStream {
    let data_struct = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => {
            return Error::new(input.span(), "Not a struct")
                .into_compile_error()
                .into()
        }
    };

    let ident = &input.ident;
    let field_impl = field_impl(&data_struct.fields);
    let field_impl_mut = field_impl_mut(&data_struct.fields);
    let field_index_impl = field_index_impl(&data_struct.fields);
    let field_index_mut_impl = field_index_mut_impl(&data_struct.fields);
    let field_count = data_struct.fields.len();

    quote! {
        #[automatically_derived]
        impl ::ryo_reflect::r#struct::Struct for #ident {
            fn field(&self, name: &str) -> Option<&dyn ::ryo_reflect::reflect::Reflect> {
                match name {
                    #field_impl
                    _ => None,
                }
            }

            fn field_mut(&mut self, name: &str) -> Option<&mut dyn ::ryo_reflect::reflect::Reflect> {
                match name {
                    #field_impl_mut
                    _ => None,
                }
            }

            fn field_index(&self, index: usize) -> Option<&dyn ::ryo_reflect::reflect::Reflect> {
                match index {
                    #field_index_impl
                    _ => None,
                }
            }

            fn field_index_mut(&mut self, index: usize) -> Option<&mut dyn ::ryo_reflect::reflect::Reflect> {
                match index {
                    #field_index_mut_impl
                    _ => None,
                }
            }

            fn field_count(&self) -> usize {
                #field_count
            }
        }
    }
}

fn field_impl(fields: &Fields) -> proc_macro2::TokenStream {
    let field_impl = fields.iter().map(|field| {
        let ident = field.ident.clone().unwrap();

        quote! {
            #ident => Some(&self.#ident),
        }
    });

    quote!(#(#field_impl)*)
}

fn field_impl_mut(fields: &Fields) -> proc_macro2::TokenStream {
    let field_impl = fields.iter().map(|field| {
        let ident = field.ident.clone().unwrap();

        quote! {
            #ident => Some(&mut self.#ident),
        }
    });

    quote!(#(#field_impl)*)
}

fn field_index_impl(fields: &Fields) -> proc_macro2::TokenStream {
    let field_index_impl = fields.iter().enumerate().map(|(index, field)| {
        let ident = field.ident.clone().unwrap();
        
        quote! {
            #index => Some(&self.#ident),
        }
    });

    quote!(#(#field_index_impl)*)
}

fn field_index_mut_impl(fields: &Fields) -> proc_macro2::TokenStream {
    let field_index_mut_impl = fields.iter().enumerate().map(|(index, field)| {
        let ident = field.ident.clone().unwrap();
        
        quote! {
            #index => Some(&mut self.#ident),
        }
    });

    quote!(#(#field_index_mut_impl)*)
}