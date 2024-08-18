mod r#enum;
mod r#struct;
mod r#union;

use proc_macro::TokenStream;
use r#enum::derive_enum_input;
use r#struct::derive_struct_input;
use r#union::derive_union_input;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    match input.data {
        Data::Struct(_) => derive_struct_input(input),
        Data::Enum(_) => derive_enum_input(input),
        Data::Union(_) => derive_union_input(input),
    }
}

#[proc_macro_derive(Struct)]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    derive_struct_input(parse_macro_input!(input))
}

#[proc_macro_derive(Enum)]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    derive_enum_input(parse_macro_input!(input))
}

#[proc_macro_derive(Union)]
pub fn derive_union(input: TokenStream) -> TokenStream {
    derive_union_input(parse_macro_input!(input))
}
