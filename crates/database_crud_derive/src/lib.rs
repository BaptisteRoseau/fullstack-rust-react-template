mod parse;
mod table_name;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use parse::ModelInfo;

#[proc_macro_derive(Crud)]
pub fn derive_crud(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _model = ModelInfo::from_derive_input(&input);

    // Code generators will be added in Chunk 4
    TokenStream::new()
}
