mod wrapper_list;
mod wrapper_map;
mod ts_enum;

mod listlike;
mod util;
pub(crate) use util::*;

use proc_macro::TokenStream;

/// Wrap a Rust `Vec` into a new type that you can `wasm_bindgen`
/// 
/// This will generate an `impl From<WasmType> for RustType` as well as a `WasmType::into_js_array`
/// 
/// # Examples
/// 
/// ```
/// # use wasm_bindgen_helper_macros::wrapper_list;
/// use wasm_bindgen::prelude::*;
/// 
/// #[wasm_bindgen]
/// /// Documentation for the generated typescript file
/// pub struct PairOfNumbers {
///     pub left: u8,
///     pub right: u8,
/// }
/// 
/// wrapper_list! {
///     #[wasm_bindgen]
///     pub struct ListOfPairs {
///         inner: std::vec::IntoIter<PairOfNumbers>,
///     }
/// }
/// 
/// #[wasm_bindgen]
/// /// This will return a javascript array typed `PairOfNumbers[]`
/// pub fn returns_an_array() -> PairOfNumbersArray {
///     let list = ListOfPairs::from(vec![
///         PairOfNumbers { left: 1, right: 2 },
///         PairOfNumbers { left: 4, right: 2 },
///     ]);
///     list.into_js_array()
/// }
/// ```
#[proc_macro]
pub fn wrapper_list(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input);
    wrapper_list::wrapper_list_impl(ast)
}

/// Wrap a Rust `Vec` into a new type that you can `wasm_bindgen`
/// 
/// This will generate an `impl From<WasmType> for RustType` as well as a `WasmType::into_js_object`
/// 
/// # Examples
/// 
/// ```
/// # use wasm_bindgen_helper_macros::wrapper_map;
/// use wasm_bindgen::prelude::*;
/// 
/// #[wasm_bindgen]
/// /// Documentation for the generated typescript file
/// pub struct LetterNumber {
///     #[wasm_bindgen(getter_with_clone)]
///     pub letter: String,
///     pub number: u32,
/// }
/// 
/// wrapper_map! {
///     #[wasm_bindgen]
///     /// Documentation for the generated typescript file
///     pub struct LettersToNumbers {
///         inner: std::vec::IntoIter<LetterNumber>,
///     }
/// }
/// 
/// #[wasm_bindgen]
/// /// This will return a javacsript object typed `{[letter: string]: number}`
/// pub fn returns_an_object() -> LetterNumberObject {
///     let a = LetterNumber { letter: "a".to_string(), number: 8 };
///     let b = LetterNumber { letter: "b".to_string(), number: 4 };
///     let list = LettersToNumbers::from(vec![a, b]);
///     let key_property = "letter";
///     list.into_js_object(key_property)
/// }
/// ```
#[proc_macro]
pub fn wrapper_map(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input);
    wrapper_map::wrapper_map_impl(ast)
}

/// Define a Rust enum with `wasm_bindgen` that allows you to return the corresponding type in typescript definitions
/// 
/// This will generate a `RustType::into_js_enum`
/// 
/// # Examples
/// 
/// ```
/// # use wasm_bindgen_helper_macros::ts_enum;
/// use wasm_bindgen::prelude::*;
/// 
/// ts_enum! {
///     #[wasm_bindgen]
///     #[derive(Clone, Copy)]
///     /// Documentation for the generated typescript file
///     pub enum Colour {
///         Green,
///         Blue,
///     }
/// }
/// 
/// #[wasm_bindgen]
/// pub fn the_better_colour() -> ColourEnum {
///     Colour::Green.into_js_enum()
/// }
/// ```
#[proc_macro]
pub fn ts_enum(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input);
    ts_enum::ts_enum_impl(ast)
}
