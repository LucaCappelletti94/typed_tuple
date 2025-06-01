//! # Typed Tuple
//! Type based operations on primitive tuple elements.
//!
//! ## Functionality
//!
//! `typed_tuple` allows for type safe operations on primitive tuple elements
//! without specifying an index. The main purpose of this crate is to simplfy
//! small arbitrary operations on heterogenous sequences. In the example below,
//! elements of a tuple are assigned and retrieved irrespective of indices:
//!
//! ```
//! # use typed_tuple::TypedTuple;
//! # #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
//! # struct Type0;
//! # #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
//! # struct Type1;
//! # #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
//! # struct Type2;
//! let mut tuple: (usize, Option<Type0>, Option<Type1>, Option<Type2>) = Default::default();
//!
//! // Mutate the `usize` element.
//! tuple.map(|el: usize| el + 10);;
//! // Assign the `Type` prefixed elements.
//! *tuple.get_mut() = Some(Type0);
//! *tuple.get_mut() = Some(Type2);
//!
//! // Pass elements to their respective consumers.
//! if let Some(element) = tuple.get() { std::hint::black_box::<Type0>(*element); }
//! if let Some(element) = tuple.get() { std::hint::black_box::<Type1>(*element); }
//! if let Some(element) = tuple.get() { std::hint::black_box::<Type2>(*element); }
//!
//! assert_eq!(tuple, (10, Some(Type0), None, Some(Type2)));
//! ```
//!
//! ## Limitations
//!
//! - Fields of the same type must still specify a constant index. This can be specified
//! with, for example, `TypedTuple::<1, _>::get(&tuple)` where `1` is the element index,
//! however this offers no advantage over simply calling `tuple.1`.
//! - `typed_tuple` can impact readability. Types should be explicit if not immediately
//! obvious. Prefer `let a: usize = tuple.get()` over `let a = tuple.get()`.
//! - `TypedTuple` is implemented on tuples of up to 12 elements in length. This was chosen
//! as it is the limit of many tuple trait implementations (`PartialEq`, `Eq`, etc.),
//! however can be extended to support a higher number of elements if needed.

/// Trait for tuple element manipulation by type.
pub trait TypedTuple<const INDEX: usize, T> {
    /// Get a reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedTuple;
    /// // Get by type.
    /// let tuple = ("a", 'b', 2usize);
    /// let a: &&str = tuple.get();
    /// let b: &char = tuple.get();
    /// let c: &usize = tuple.get();
    ///
    /// // Get by 'const' index.
    /// let a = TypedTuple::<0, _>::get(&tuple);
    /// let b = TypedTuple::<1, _>::get(&tuple);
    /// let c = TypedTuple::<2, _>::get(&tuple);
    /// ```
    fn get(&self) -> &T;

    /// Get a mutable reference to the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedTuple;
    /// // Mutate by type.
    /// let mut tuple = ("a", 'b', 2usize);
    /// *tuple.get_mut() = "c";
    /// *tuple.get_mut() = 'd';
    /// *tuple.get_mut() = 3usize;
    /// assert_eq!(tuple, ("c", 'd', 3));
    ///
    /// // Mutate by 'const' index.
    /// *TypedTuple::<0, _>::get_mut(&mut tuple) = "e";
    /// *TypedTuple::<1, _>::get_mut(&mut tuple) = 'f';
    /// *TypedTuple::<2, _>::get_mut(&mut tuple) = 4usize;
    /// assert_eq!(tuple, ("e", 'f', 4))
    /// ```
    fn get_mut(&mut self) -> &mut T;

    /// Takes a closure, mutating the element of type `T`.
    /// # Example
    /// ```
    /// # use typed_tuple::TypedTuple;
    /// // Map by type.
    /// let mut tuple = ("a".to_string(), 1u8, 2usize);
    /// tuple.map(|el: String| el.to_uppercase());
    /// tuple.map(|el: u8| el + 1);
    /// tuple.map(|el: usize| el + 2);
    /// assert_eq!(tuple, ("A".to_string(), 2, 4));
    ///
    /// // Map by 'const' index.
    /// TypedTuple::<0, _>::map(&mut tuple, |el| el.to_lowercase());
    /// TypedTuple::<1, _>::map(&mut tuple, |el| el - 1);
    /// TypedTuple::<2, _>::map(&mut tuple, |el| el - 2);
    /// assert_eq!(tuple, ("a".to_string(), 1, 2))
    /// ```
    fn map<FN: FnOnce(T) -> T>(&mut self, f: FN)
    where
        T: Default;
}

macro_rules! impl_typed_tuple {
    (( $($generics:tt ),* ), [ $( $( $idx_tail:tt ),+ )? ], []) => {};

    (( $($generics:tt ),* ), [$idx_head:tt  $(, $idx_tail:tt )* ], [ $gen_head:tt $(, $gen_tail:tt )* ]) => {
        impl< $( $generics ),+ > TypedTuple<$idx_head, $gen_head> for ( $( $generics ),+ ) {
            fn get(&self) -> &$gen_head {
                &self.$idx_head
            }

            fn get_mut(&mut self) -> &mut $gen_head {
                &mut self.$idx_head
            }

            fn map<FN: FnOnce($gen_head) -> $gen_head>(&mut self, f: FN) where $gen_head: Default {
                self.$idx_head = f(std::mem::take(&mut self.$idx_head));
            }
        }
        impl_typed_tuple!(($( $generics ),* ), [ $( $idx_tail ),* ], [ $( $gen_tail ),* ]);
    };

    (( $($generics:tt),* )) => {
        impl_typed_tuple!(
            ( $( $generics ),* ),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [ $( $generics ),* ]);
    }
}

impl_typed_tuple!((A, B));
impl_typed_tuple!((A, B, C));
impl_typed_tuple!((A, B, C, D));
impl_typed_tuple!((A, B, C, D, E));
impl_typed_tuple!((A, B, C, D, E, F));
impl_typed_tuple!((A, B, C, D, E, F, G));
impl_typed_tuple!((A, B, C, D, E, F, G, H));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I, K));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I, K, J));
impl_typed_tuple!((A, B, C, D, E, F, G, H, I, K, J, L));
