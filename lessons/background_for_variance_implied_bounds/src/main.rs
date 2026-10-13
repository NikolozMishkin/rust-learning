struct GenericWithLifetime<'a, T> {
    b: &'a T, // implied T: 'a   // 'a: 'b => 'a outlives 'b and now T: 'a => T outlives 'a
}

fn lifetime_bound_implied_fn<'a, T>(x: GenericWithLifetime<T>) {} // implied T: 'a

fn lifetime_bound_implied_fn_1<'b, U>(x: &'b U) {} // implied U: 'b

fn requires_lifetime_bound<'a, T>() {} // bound not not implied

trait Trait<'a, T: 'a> {}
//impl<'a, T> Trait<'a, T> for () {} // not implied T: 'a

// This compiles as `T: 'a` is implied by the self type `&'a T`.
impl<'a, T> Trait<'a, T> for &'a T {} // implied T: 'a

struct A<T: Clone>(T);

fn doesnt_specify_T_Clone<T: Clone>(x: A<T>) {} // add the lifetime bound of T: Clone
fn main() {}

/*
-------------------------------------------------------------------------------------------------------------
Concept / Topic               | Explanation
------------------------------|------------------------------------------------------------------------------
Implied Lifetime Bounds       | Lifetime bounds can be inferred by the compiler.
                              | They are not always be written explicitly.
                              | These bounds are derived from the structure of types themselves.

Well-Formed Types             | A type is well-formed if all referenced types outlive the required lifetimes.

Where Bounds Are Inferred     | Typically in function parameters and structs containing references.

Where Bounds Are Not Inferred | When there is no input type for function enforcing the relationship.
                              | In trait implementations without reference-based context.

Lifetime vs Trait Bounds      | Only lifetime bounds are inferred by the compiler.
                              | Trait bounds must always be specified explicitly by the programmer.
-------------------------------------------------------------------------------------------------------------
*/
