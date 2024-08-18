use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error};

pub(crate) fn derive_enum_input(input: DeriveInput) -> TokenStream {
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Error::new(input.span(), "Not an enum")
                .into_compile_error()
                .into()
        }
    };

    todo!()
}
