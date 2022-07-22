use quote::quote;

macro_rules! expect_match {
    ($lhs:expr, $rhs:pat, $extract:expr) => {
        if let $rhs = $lhs { $extract } else { panic!("Invalid input"); }
    }
}
pub(crate) use expect_match;

pub(crate) fn parse_attrs(input: &syn::DeriveInput) -> quote::__private::TokenStream {
    let attrs = &input.attrs;
    let bindgen_attr = if !attrs.iter().any(|attr| attr.path.segments.first().map_or(false, |seg| seg.ident == "wasm_bindgen")) {
        quote! { #[wasm_bindgen] }
    } else { quote! {} };
    quote! { #bindgen_attr #(#attrs)* }
}
