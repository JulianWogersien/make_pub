use proc_macro::TokenStream;
use syn::Fields;
use syn::Visibility;
use syn::parse_macro_input;
use syn::ItemStruct;
use quote::quote;

#[proc_macro_attribute]
pub fn make_pub(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(_input as ItemStruct);
    
    if let Fields::Named(ref mut fields) = input.fields {
        for field in &mut fields.named {
            field.vis = Visibility::Public(Default::default());
        }
    }

    TokenStream::from(quote! {
        #input
    })
}