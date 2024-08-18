use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error};

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(input: TokenStream) -> TokenStream {
    let derive_input = input.clone();
    let derive_input: DeriveInput = parse_macro_input!(derive_input);

    match &derive_input.data {
        Data::Struct(_) => derive_struct(input),
        Data::Enum(_) => derive_enum(input),
        Data::Union(_) => derive_enum(input),
    }
}

#[proc_macro_derive(Struct)]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let data_struct = match input.data {
        Data::Struct(data_struct) => data_struct,
        _ => {
            return Error::new(input.span(), "Not a struct")
                .into_compile_error()
                .into()
        }
    };

    todo!()
}

#[proc_macro_derive(Enum)]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let data_enum = match input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Error::new(input.span(), "Not an enum")
                .into_compile_error()
                .into()
        }
    };

    todo!()
}

#[proc_macro_derive(Union)]
pub fn derive_union(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let data_union = match input.data {
        Data::Enum(data_union) => data_union,
        _ => {
            return Error::new(input.span(), "Not a union")
                .into_compile_error()
                .into()
        }
    };

    todo!()
}
