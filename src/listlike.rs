use quote::quote;

use crate::expect_match;

pub(crate) struct ListLike {
    pub tokens: quote::__private::TokenStream,
    pub list_ident: syn::Ident,
    pub item_ident: syn::Ident,
}

pub(crate) fn listlike_impl(input: syn::DeriveInput) -> ListLike {
    let attrs = crate::parse_attrs(&input);

    let list_ident = input.ident;

    let fields = expect_match!(input.data, syn::Data::Struct(data_struct), data_struct.fields);
    assert!(fields.len() < 2, "Too many fields");
    let field = expect_match!(fields, syn::Fields::Named(mut fields), fields.named.pop().expect("Expected one field").into_value());

    let inner_ident = field.ident.unwrap();
    let last_path_segment = expect_match!(field.ty, syn::Type::Path(mut path), path.path.segments.pop().unwrap().into_value());
    assert_eq!(last_path_segment.ident, "IntoIter", "Expected field of type `IntoIter`");
    let iter_arg = expect_match!(last_path_segment.arguments, syn::PathArguments::AngleBracketed(mut args), args.args.pop().unwrap().into_value());
    let item_ident = expect_match!(iter_arg, syn::GenericArgument::Type(syn::Type::Path(mut path)), path.path.segments.pop().unwrap().into_value().ident);

    let tokens = quote! {
        #attrs
        pub struct #list_ident {
            #inner_ident: ::std::vec::IntoIter<#item_ident>,
        }
        impl ::std::convert::From<::std::vec::Vec<#item_ident>> for #list_ident {
            fn from(vec: ::std::vec::Vec<#item_ident>) -> #list_ident {
                #list_ident { #inner_ident: vec.into_iter() }
            }
        }
        #[wasm_bindgen]
        impl #list_ident {
            #[allow(clippy::should_implement_trait)]
            pub fn next(&mut self) -> ::std::option::Option<#item_ident> {
                self.#inner_ident.next()
            }
        }
    };

    ListLike { tokens, list_ident, item_ident }
}
