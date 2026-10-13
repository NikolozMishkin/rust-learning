// 'b: 'a -> 'b outlives 'a or 'b lasts atlest as long as 'a
fn largest<'a, 'b>(x: &'a u8, y: &'b u8) -> &'a u8
where
    'b: 'a,
{
    if x > y { x } else { y }
}

fn main() {}

/*
------------------------------------------------------------------------------------------------------------
Concept / Topic           | Explanation
--------------------------|----------------------------------------------------------------------------------
Lifetime Elision Failure  | Lifetime elision rules may fail to resolve the output lifetimes.
                          | For instance, when multiple input reference lifetimes exist.
                          | In such cases, lifetimes must be specified explicitly.

Manual Lifetime Annotation| Explicitly annotating the return lifetime is required when elision fails.
                          | Assigning the return lifetime to one parameter, may still be insufficient.

Lifetime Bounds           | Lifetime bounds express relationships between multiple lifetimes.
                          | A bound like 'b: 'a means lifetime 'b outlives lifetime 'a.

Compiler Acceptance Logic | A returned reference is accepted, if its lifetime is at least equal to shorter lifetime.
                          | Returning a longer lifetime from a function is acceptable but returning shorter is not.
-------------------------------------------------------------------------------------------------------------
*/
