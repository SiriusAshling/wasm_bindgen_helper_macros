use proc_macro::TokenStream;
use quote::quote;

use crate::listlike::*;

pub fn wrapper_list_impl(input: syn::DeriveInput) -> TokenStream {
    let ListLike { tokens, list_ident, item_ident } = listlike_impl(input);

    let js_ident = quote::format_ident!("__toArray{}", list_ident);
    let inline_js = format!("\
        function {js_ident}(wasmList) {{\
            const items = [];\
            while (true) {{\
                const item = wasmList.next();\
                if (item === undefined) {{\
                    return items;\
                }}\
                items.push(item);\
            }}\
        }}\
        module.exports = {{ {js_ident} }}\
    ", js_ident = js_ident);
    let rust_return_type = quote::format_ident!("{}Array", item_ident);
    let ts_return_type = format!("{}[]", item_ident);

    quote! {
        #tokens
        #[wasm_bindgen(inline_js = #inline_js)]
        extern "C" {
            #[wasm_bindgen(typescript_type = #ts_return_type)]
            pub type #rust_return_type;

            pub(crate) fn #js_ident(list: #list_ident) -> #rust_return_type;
        }
        #[wasm_bindgen]
        impl #list_ident {
            pub(crate) fn into_js_array(self) -> #rust_return_type {
                #js_ident(self)
            }
        }
    }.into()
}
