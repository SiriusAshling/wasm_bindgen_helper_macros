use proc_macro::TokenStream;
use quote::quote;

use crate::expect_match;

pub fn ts_enum_impl(input: syn::DeriveInput) -> TokenStream {
    let attrs = crate::parse_attrs(&input);

    let enum_ident = input.ident;

    let variants = expect_match!(input.data, syn::Data::Enum(data_enum), data_enum.variants);

    let js_ident = quote::format_ident!("__reinterpret{}", enum_ident);
    let inline_js = format!("\
        function {js_ident}(thing) {{\
            return thing;\
        }}\
        module.exports = {{ {js_ident} }}\
    ", js_ident = js_ident);
    let rust_return_type = quote::format_ident!("{}Enum", enum_ident);

    quote! {
        #attrs
        pub enum #enum_ident {
            #variants
        }
        #[wasm_bindgen(inline_js = #inline_js)]
        extern "C" {
            #[wasm_bindgen(typescript_type = #enum_ident)]
            pub type #rust_return_type;
            pub(crate) fn #js_ident(thing: u32) -> #rust_return_type;
        }
        impl #enum_ident {
            pub(crate) fn into_js_enum(self) -> #rust_return_type {
                #js_ident(self as u32)
            }
        }
    }.into()
}
