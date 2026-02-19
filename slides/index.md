---
description: GATs, Trait Bounds, and Higher-Ranked Trait Bounds
marp: true
paginate: true
theme: default
title: Hands-On with Advanced Rust Types
---

# Hands-On with Advanced Rust Types

## GATs, Trait Bounds

## and Higher-Ranked Trait Bounds

Vitaly Bragilevsky @ RustNation UK 2026

------------------------------------------------------------------------

# Why Types Matter

Types are not annotations.\
Types are **constraints on possible programs**.

They help us:

-   Prevent invalid states
-   Move errors to compile time
-   Express invariants
-   Encode guarantees about memory and behavior

------------------------------------------------------------------------

<style scoped>
table {
    height: 100%;
    width: 100%;
    font-size: 40px;
}
th {
    color: blue;
}
</style>

# Evolution of Type Expressiveness

| Feature          | What It Adds                      |
|------------------|-----------------------------------|
| Static types     | Data shape                        |
| Generics         | Reusable algorithms               |
| Trait bounds     | Behavioral constraints            |
| Associated types | Type-level computation            |
| GAT              | Type constructors                 |
| HRTB             | Universal lifetime quantification |

------------------------------------------------------------------------

# Rust's Type System Is About

-   Behavior (`trait`)
-   Ownership
-   Lifetimes
-   Zero-cost abstractions
-   Expressing invariants without runtime cost

------------------------------------------------------------------------

# Today's Focus

-   Trait bounds
-   Generic associated types (GAT)
-   Higher-ranked trait bounds (HRTB)

---

# 1️⃣ Generics Refresher

``` rust
fn identity<T>(x: T) -> T {
    x
}
```

Parametric polymorphism:

-   One algorithm
-   Many types

------------------------------------------------------------------------

# Trait Bounds

``` rust
fn print<T: std::fmt::Display>(x: T) {
    println!("{x}");
}
```

Types describe **behavior**, not just shape.

------------------------------------------------------------------------

# Multiple Bounds

``` rust
fn foo<T>(x: T)
where
    T: Clone + Send + 'static,
{
}
```

------------------------------------------------------------------------

# Associated Types

``` rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Associated types are defined by implementations and depend on `Self`.

------------------------------------------------------------------------

# Associated Types as Type-Level Output

``` rust
trait Parser {
    type Output;
}
```

Think of it as:

Parser → Type

------------------------------------------------------------------------

# Enter GAT

## Generic Associated Types

``` rust
trait Foo {
    type Bar<T>;
}
```

Now:

Bar: Type → Type

------------------------------------------------------------------------

# GAT with Type Parameters

``` rust
trait Storage {
    type Ref<T>;
    fn wrap<T>(value: T) -> Self::Ref<T>;
}
```

Example implementation:

``` rust
struct Shared;

impl Storage for Shared {
    type Ref<T> = std::sync::Arc<T>;
    fn wrap<T>(value: T) -> Self::Ref<T> {
        std::sync::Arc::new(value)
    }
}
```

------------------------------------------------------------------------

# Functor-Like Example

``` rust
pub trait Mappable {
    type Item;
    type Mapped<U>;

    fn map<U, F: FnMut(Self::Item) -> U>(self, f: F) -> Self::Mapped<U>;
}
```

------------------------------------------------------------------------

# GAT with Lifetimes

``` rust
trait LendingIterator {
    type Item<'a>;
    fn next<'a>(&'a mut self) -> Self::Item<'a>;
}
```

Item: Lifetime → Type

------------------------------------------------------------------------

# GAT with Type + Lifetime

``` rust
trait Arena {
    type Handle<'a, T>;
    fn alloc<'a, T>(&'a self, value: T)
        -> Self::Handle<'a, T>;
}
```

Handle: (Lifetime, Type) → Type

------------------------------------------------------------------------

# Summary So Far

| Feature       | Meaning                     |
|---------------|-----------------------------|
| Associated type | Fixed output type           |
| `GAT<T>`      | Type constructor            |
| `GAT<'a>`     | Lifetime-dependent type     |
| `GAT<'a, T>`   | General type-level function |

------------------------------------------------------------------------

# Higher-Ranked Trait Bounds

``` rust
fn foo<F>(f: F)
where
    F: for<'a> Fn(&'a u32),
{
}
```

∀ 'a. Fn(&'a u32)

------------------------------------------------------------------------

# Why HRTB Matters

``` rust
fn call_on_ref_zero<F>(f: F)
where
    F: for<'a> Fn(&'a i32),
{
    let x = 0;
    f(&x);
}
```

We require universal lifetime validity.

------------------------------------------------------------------------

# GAT vs HRTB

| GAT            | HRTB                 |
|----------------|----------------------|
| Produces types | Constrains lifetimes |
| `type Foo<'a>` | `for<'a>`            |

------------------------------------------------------------------------

# Real-World Impact

-   Lending iterators
-   Zero-copy parsers
-   Async ecosystem
-   ECS & arena APIs
-   High-level abstractions without runtime cost

------------------------------------------------------------------------

# Final Takeaway

Rust's type system lets us express:

-   Behavior
-   Ownership
-   Lifetime relations
-   Type-level functions
-   Universal lifetime guarantees

All enforced at compile time.

------------------------------------------------------------------------

# Let's Code!

`github.com/bravit/rustnationuk26-rust-types-hands-on`

![height:500px](qr.png)