/// Define a Rust enum with `wasm_bindgen` that allows you to return the corresponding type in typescript definitions
/// 
/// This will generate a `RustType::into_js_enum`
/// 
/// # Examples
/// 
/// ```
/// # mod module {
/// # use wasm_bindgen_util_macros::ts_enum;
/// use wasm_bindgen::prelude::*;
/// 
/// ts_enum! {
///     #[wasm_bindgen]
///     #[derive(Clone, Copy)]
///     #[doc = " Documentation for the generated typescript file"]
///     pub enum Colour {
///         Green,
///         Blue,
///     }
///     mod colour { typescript_type = "Colour" }
/// }
/// 
/// #[wasm_bindgen]
/// pub fn the_better_colour() -> colour::ReturnEnum {
///     Colour::Green.into_js_enum()
/// }
/// # }
/// ```
#[macro_export]
macro_rules! ts_enum {
    (
        $(#[$attributes:meta])*
        pub enum $wasm_type:ident $body:tt
        mod $mod_name:ident { typescript_type = $return_ty:literal }
    ) => {
        $(#[$attributes])*
        pub enum $wasm_type $body
        mod $mod_name {
            use super::$wasm_type;
            use wasm_bindgen::prelude::wasm_bindgen;
            #[wasm_bindgen(inline_js = "\
                function reinterpret(thing) {\
                    return thing;\
                }\
                module.exports = { reinterpret }\
            ")]
            extern "C" {
                #[wasm_bindgen(typescript_type = $return_ty)]
                pub type ReturnEnum;
                pub(crate) fn reinterpret(thing: u32) -> ReturnEnum;
            }
            impl $wasm_type {
                pub(crate) fn into_js_enum(self) -> ReturnEnum {
                    reinterpret(self as u32)
                }
            }
        }
    }
}

/// Wrap a Rust type into a new type that you can `wasm_bindgen`
/// 
/// This will generate an `impl From<WasmType> for RustType`
/// 
/// # Examples
/// 
/// ```
/// # use wasm_bindgen_util_macros::wrapper_type;
/// use wasm_bindgen::prelude::*;
/// 
/// wrapper_type! {
///     #[wasm_bindgen]
///     #[doc = " Documentation for the generated typescript file"]
///     pub struct PairOfNumbers {
///         inner: (u8, u8),
///     }
/// }
/// 
/// let pair = PairOfNumbers::from((6, 9));
/// 
/// assert_eq!(pair.inner, (6, 9))
/// ```
#[macro_export]
macro_rules! wrapper_type {
    (
        $(#[$attribute:meta])*
        pub struct $wasm_type:ident {
            $field:ident: $wrapped_type:ty,
        }
    ) => {
        $(#[$attribute])*
        pub struct $wasm_type {
            $field: $wrapped_type,
        }
        impl From<$wrapped_type> for $wasm_type {
            fn from($field: $wrapped_type) -> $wasm_type {
                $wasm_type { $field }
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __listlike {
    (
        $(#[$attributes:meta])*
        pub struct $wasm_type:ident {
            $field:ident: ::std::vec::IntoIter<$wrapped_type:ty>$(,)?
        }
    ) => {
        $(#[$attributes])*
        pub struct $wasm_type {
            $field: ::std::vec::IntoIter<$wrapped_type>
        }
        impl ::std::convert::From<::std::vec::Vec<$wrapped_type>> for $wasm_type {
            fn from($field: ::std::vec::Vec<$wrapped_type>) -> $wasm_type {
                $wasm_type { $field: $field.into_iter() }
            }
        }
        #[wasm_bindgen]
        impl $wasm_type {
            #[allow(clippy::should_implement_trait)]
            pub fn next(&mut self) -> ::std::option::Option<$wrapped_type> {
                self.$field.next()
            }
        }
    }
}
/// Wrap a Rust `Vec` into a new type that you can `wasm_bindgen`
/// 
/// This will generate an `impl From<WasmType> for RustType` as well as a `WasmType::into_js_array`
/// 
/// # Examples
/// 
/// ```
/// # mod module {
/// # use wasm_bindgen_util_macros::wrapper_type;
/// # use wasm_bindgen_util_macros::wrapper_list;
/// use wasm_bindgen::prelude::*;
/// 
/// wrapper_type! {
///     #[wasm_bindgen]
///     #[doc = " Documentation for the generated typescript file"]
///     pub struct PairOfNumbers {
///         inner: (u8, u8),
///     }
/// }
/// 
/// wrapper_list! {
///     #[wasm_bindgen]
///     pub struct ListOfPairs {
///         inner: std::vec::IntoIter<PairOfNumbers>,
///     }
///     mod list_of_pairs { typescript_type = "PairOfNumbers[]" }
/// }
/// 
/// #[wasm_bindgen]
/// /// This will return a javascript array typed `PairOfNumbers[]`
/// pub fn returns_an_array() -> list_of_pairs::ReturnArray {
///     let list = ListOfPairs::from(vec![(6, 9).into(), (4, 2).into()]);
///     list.into_js_array()
/// }
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! wrapper_list {
    (
        $(#[$attributes:meta])*
        pub struct $wasm_type:ident {
            $field:ident: $($($(::)?std::)?vec::)?IntoIter<$wrapped_type:ty>$(,)?
        }
        mod $mod_name:ident { typescript_type = $return_ty:literal }
    ) => {
        __listlike! {
            #[doc = " Used internally to construct arrays"]
            $(#[$attributes])*
            pub struct $wasm_type {
                $field: ::std::vec::IntoIter<$wrapped_type>
            }
        }
        mod $mod_name {
            use super::$wasm_type;
            use wasm_bindgen::prelude::wasm_bindgen;
            #[wasm_bindgen(inline_js = "\
                function toArray(wasmList) {\
                    const items = [];\
                    while (true) {\
                        const item = wasmList.next();\
                        if (item === undefined) {\
                            return items;\
                        }\
                        items.push(item);\
                    }\
                }\
                module.exports = { toArray }\
            ")]
            extern "C" {
                #[wasm_bindgen(typescript_type = $return_ty)]
                pub type ReturnArray;

                pub(crate) fn toArray(list: $wasm_type) -> ReturnArray;
            }
            impl $wasm_type {
                pub(crate) fn into_js_array(self) -> ReturnArray {
                    toArray(self)
                }
            }
        }
    }
}
/// Wrap a Rust `Vec` into a new type that you can `wasm_bindgen`
/// 
/// This will generate an `impl From<WasmType> for RustType` as well as a `WasmType::into_js_object`
/// 
/// # Examples
/// 
/// ```
/// # mod module {
/// # use wasm_bindgen_util_macros::wrapper_map;
/// use wasm_bindgen::prelude::*;
/// 
/// #[wasm_bindgen]
/// #[doc = " Documentation for the generated typescript file"]
/// pub struct LetterNumber {
///     #[wasm_bindgen(getter_with_clone)]
///     pub letter: String,
///     pub number: u32,
/// }
/// 
/// wrapper_map! {
///     #[wasm_bindgen]
///     #[doc = " Documentation for the generated typescript file"]
///     pub struct LettersToNumbers {
///         inner: std::vec::IntoIter<LetterNumber>,
///     }
///     mod letters_to_numbers { typescript_type = "{[letter: string]: number}" }
/// }
/// 
/// #[wasm_bindgen]
/// /// This will return a javacsript object typed `{[letter: string]: number}`
/// pub fn returns_an_object() -> letters_to_numbers::ReturnObject {
///     let a = LetterNumber { letter: "a".to_string(), number: 8 };
///     let b = LetterNumber { letter: "b".to_string(), number: 4 };
///     let list = LettersToNumbers::from(vec![a, b]);
///     let key_property = "letter";
///     list.into_js_object(key_property)
/// }
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! wrapper_map {
    (
        $(#[$attributes:meta])*
        pub struct $wasm_type:ident {
            $field:ident: $($($(::)?std::)?vec::)?IntoIter<$wrapped_type:ty>$(,)?
        }
        mod $mod_name:ident { typescript_type = $return_ty:literal }
    ) => {
        __listlike! {
            #[doc = " Used internally to construct objects"]
            $(#[$attributes])*
            pub struct $wasm_type {
                $field: ::std::vec::IntoIter<$wrapped_type>
            }
        }
        mod $mod_name {
            use super::$wasm_type;
            use wasm_bindgen::prelude::wasm_bindgen;
            #[wasm_bindgen(inline_js = "\
                function toObject(wasmMap, keyProperty) {\
                    const object = {};\
                    while (true) {\
                        const value = wasmMap.next();\
                        if (value === undefined) {\
                            return object;\
                        }\
                        const key = value[keyProperty];\
                        object[key] = value;\
                    }\
                }\
                module.exports = { toObject }\
            ")]
            extern "C" {
                #[wasm_bindgen(typescript_type = $return_ty)]
                pub type ReturnObject;

                pub(crate) fn toObject(list: $wasm_type, key_property: &str) -> ReturnObject;
            }
            impl $wasm_type {
                pub(crate) fn into_js_object(self, key_property: &str) -> ReturnObject {
                    toObject(self, key_property)
                }
            }
        }
    }
}
