use crate::r#enum::derive_enum_input;
use crate::r#struct::derive_struct_input;
use crate::union::derive_union_input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn derive_reflect_input(input: DeriveInput) -> TokenStream {
    let ident = input.ident.clone();

    let reflect = match input.data {
        Data::Struct(_) => derive_struct_input(input),
        Data::Enum(_) => derive_enum_input(input),
        Data::Union(_) => derive_union_input(input),
    };

    quote! {
        #[automatically_derived]
        impl ::ryo_reflect::reflect::Reflect for #ident {
            fn as_any(&self) -> &dyn ::core::any::Any {
                self as &dyn ::core::any::Any
            }
        
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self as &mut dyn ::core::any::Any
            }
        
            fn as_reflect(&self) -> &dyn ::ryo_reflect::reflect::Reflect {
                self
            }
        
            fn as_reflect_mut(&mut self) -> &mut dyn ::ryo_reflect::reflect::Reflect {
                self
            }
        }

        #reflect
    }
}
