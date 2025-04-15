use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Type};

#[cfg(doctest)]
use storage;

/// Create type aliases for the provided type.
/// # Output
/// Creates types:
/// - `#TypeNameCache = Cache<TypeName>`
/// - `#TypeNameKey = CacheKey<TypeName>`
///
/// # Example
/// ```
/// use macros::create_cache_type_aliases;
/// use storage::{Cache,Cacheable};
/// use std::any::{TypeId, Any};
/// struct Wrapperi32(i32);
///
/// impl Cacheable for Wrapperi32{
///   type Output = i32;
///
///   fn hash(&self,) -> Self::Output {
///       self.0
///    }
/// }
///
/// create_cache_type_aliases!(Wrapperi32);
///
/// fn main(){
///   assert_eq!(TypeId::of::<Cache<Wrapperi32>>(), TypeId::of::<Wrapperi32Cache>());
/// }
/// ```
#[proc_macro]
pub fn create_cache_type_aliases(stream: TokenStream,) -> TokenStream {
  let ty = parse_macro_input!(stream as Type);
  let cache_name = format_ident!("{}Cache", quote!(#ty).to_string());
  let cache_key = format_ident!("{}Key", quote!(#ty).to_string());

  // Look into how to add documentation to the generated stuff
  // i.e. The key to a given [`Material`]'s entry in the [`MaterialCache`].
  let expanded = quote! {
    pub type #cache_name = storage::Cache<#ty>;
    pub type #cache_key = storage::CacheKey<#ty>;
  };

  TokenStream::from(expanded,)
}

// #[proc_macro_derive(Cacheable)]
// pub fn derive_cacheable(stream: TokenStream,) -> TokenStream {
//   // TODO: Make more general

//   let input = parse_macro_input!(stream as DeriveInput);
//   let name = input.ident;

//   let expanded = match input.data {
//     syn::Data::Struct(data,) => match &data.fields {
//       syn::Fields::Named(named,) => todo!(),
//       syn::Fields::Unnamed(unnamed,) => {
//         // If it is a single field tuple struct try to derive on the field
//         if data.fields.len() == 1 {
//           // Get the type of the field
//           let field_ty = &data.fields.iter().last().unwrap().ty;
//           quote! {
//             impl Cacheable for #name {
//               type Output = #field_ty;

//               fn hash(&self) -> Self::Output {
//                 self.0.clone()
//               }
//             }
//           }
//         } else {
//           todo!()
//         }
//       }
//       syn::Fields::Unit => todo!("Cacheable cannot be implemented for unit
// structs"),     },
//     syn::Data::Enum(data,) => {
//       // Just need some kind of value from the enum that's unique to the enum
//       todo!()
//     }
//     syn::Data::Union(data_union,) => unreachable!("Cachable cannot be derived
// for unions"),   };

//   TokenStream::from(expanded,)
// }
