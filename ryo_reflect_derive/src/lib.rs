mod r#enum;
mod reflect;
mod r#struct;
mod r#union;

use proc_macro::TokenStream;
use r#enum::derive_enum_input;
use r#struct::derive_struct_input;
use r#union::derive_union_input;
use reflect::derive_reflect_input;
use syn::parse_macro_input;

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
    derive_reflect_input(parse_macro_input!(input)).into()
}

#[proc_macro_derive(Struct)]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    derive_struct_input(parse_macro_input!(input)).into()
}

#[proc_macro_derive(Enum)]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    derive_enum_input(parse_macro_input!(input)).into()
}

#[proc_macro_derive(Union)]
pub fn derive_union(input: TokenStream) -> TokenStream {
    derive_union_input(parse_macro_input!(input)).into()
}
