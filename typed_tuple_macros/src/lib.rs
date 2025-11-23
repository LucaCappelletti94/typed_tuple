use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

/// Generates TypedTuple implementations for tuples up to the specified size.
///
/// This proc macro generates all the necessary trait implementations including
/// the `pop` method for each tuple size and index combination.
#[proc_macro]
pub fn generate_typed_tuple_impls(input: TokenStream) -> TokenStream {
    let max_size = parse_macro_input!(input as LitInt);
    let max_size: usize = max_size.base10_parse().expect("Expected a number");

    let mut impls = Vec::new();

    // Generate the marker types for each index, since at the time of writing
    // Rust does not support const generics in traits fully.
    // See: <https://github.com/rust-lang/rust/issues/76560>

    let mut index_idents = Vec::new();
    for index in 0..max_size {
        let marker_ident = quote::format_ident!("TupleIndex{}", index);
        index_idents.push(marker_ident.clone());
        let documentation = format!("Marker type for tuple index {}", index);
        impls.push(quote! {
            #[doc = #documentation]
            pub struct #marker_ident;
        });
    }

    // Generate implementation for each tuple size
    for size in 1..=max_size {
        let type_params: Vec<_> = (0..size).map(|i| quote::format_ident!("T{}", i)).collect();

        // Generate implementation for each index in the tuple
        for (target_type, (index, index_marker)) in
            type_params.iter().zip(index_idents.iter().enumerate())
        {
            let index_lit = syn::Index::from(index);

            // Build type and index lists for pop, split_at
            let pop_output_types =
                type_params.iter().enumerate().filter(|(i, _)| *i != index).map(|(_, t)| t);
            let remaining_indices = (0..size).filter(|i| *i != index).map(syn::Index::from);

            let split_left_types = type_params.iter().take(index + 1);
            let split_right_types = type_params.iter().skip(index + 1);
            let split_left_indices = (0..=index).map(syn::Index::from);
            let split_right_indices = ((index + 1)..size).map(syn::Index::from);

            impls.push(quote! {
                impl<#(#type_params),*> TypedTuple<#index_marker, #target_type> for (#(#type_params,)*) {
                    type PopOutput = (#(#pop_output_types,)*);
                    type SplitLeft = (#(#split_left_types,)*);
                    type SplitRight = (#(#split_right_types,)*);
                    const INDEX: usize = #index;

                    #[inline]
                    fn get(&self) -> &#target_type {
                        &self.#index_lit
                    }
                    #[inline]
                    fn get_mut(&mut self) -> &mut #target_type {
                        &mut self.#index_lit
                    }
                    #[inline]
                    fn map<FN: FnOnce(#target_type) -> #target_type>(&mut self, f: FN)
                    where
                        #target_type: Default
                    {
                        self.#index_lit = f(core::mem::take(&mut self.#index_lit));
                    }
                    #[inline]
                    fn pop(self) -> (#target_type, Self::PopOutput) {
                        (self.#index_lit, (#(self.#remaining_indices,)*))
                    }
                    #[inline]
                    fn split_at(self) -> (Self::SplitLeft, Self::SplitRight) {
                        ((#(self.#split_left_indices,)*), (#(self.#split_right_indices,)*))
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}
