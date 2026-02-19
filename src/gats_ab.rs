// Your goal: make a trait that abstracts “map over container”.
// The key is the GAT: `type Mapped<U>` describes the *same shape* with `U` inside.
// Source of this assignment (don't read before solving it):
// https://www.reddit.com/r/rust/comments/ynvm8a/comment/ivc5n8d/

pub trait Mappable {
    type Item;
    type Mapped<U>;

    fn map<U, F: FnMut(Self::Item) -> U>(self, f: F) -> Self::Mapped<U>;
}

// === Part A: implement Mappable for Option<T>, Result<T, E>, Vec<T> ===

// TODO Option<T>

// TODO Result<T, E>

// TODO Vec<T>

// === Part B: implement Mappable for fixed-size arrays [T; N] =================
// Goal: `[T; N]` -> `[U; N]` without excessive heap allocation.
// *You must keep the length N.*

// TODO [T; N]

// === Utility functions for the demo/tests =====================================

pub fn zero_to_42<C: Mappable<Item = i32>>(c: C) -> C::Mapped<i32> {
    c.map(|x| if x == 0 { 42 } else { x })
}

pub fn to_string_excited<T: ToString, C: Mappable<Item = T>>(c: C) -> C::Mapped<String> {
    c.map(|x| x.to_string() + "!")
}

// === Tests ===================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_and_result() {
        let o0: Option<i32> = Some(0);
        let o43: Option<i32> = Some(43);
        assert_eq!(zero_to_42(o0), Some(42));
        assert_eq!(zero_to_42(o43), Some(43));
        assert_eq!(to_string_excited(o43), Some("43!".to_string()));

        let r0: Result<i32, ()> = Ok(0);
        let r43: Result<i32, ()> = Ok(43);
        assert_eq!(zero_to_42(r0), Ok(42));
        assert_eq!(zero_to_42(r43), Ok(43));
        assert_eq!(to_string_excited(r43), Ok("43!".to_string()));
    }

    #[test]
    fn vec_mapping() {
        let v = vec![0, 1, 2];
        assert_eq!(zero_to_42(v), vec![42, 1, 2]);
        let v1 = vec![0.5, 1.5, 2.5];
        assert_eq!(to_string_excited(v1), vec!["0.5!", "1.5!", "2.5!"]);
    }

    #[test]
    fn array_mapping() {
        let a = [0, 1, 2, 3];
        let b = zero_to_42(a);
        assert_eq!(b, [42, 1, 2, 3]);
        // Type check: must remain an array of the same length:
        let _: [i32; 4] = b;
        let c = to_string_excited(b);
        assert_eq!(c, ["42!", "1!", "2!", "3!"]);
    }
}