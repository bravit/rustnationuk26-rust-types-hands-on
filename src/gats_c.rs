// === Part C: add a by-reference variant ======================================
// Sometimes you want to map over references instead of moving items out.
// Define a *second* trait that uses a GAT with a lifetime to express borrowing.
//
// Requirements:
// - name it `RefMappable`
// - associated items:
//     type Item<'a>  (the thing you get when borrowing)
//     type Result<U> (the result container when mapping by reference)
// - method:
//     fn map_ref<'a, U, F>(&'a self, f: F) -> Self::Result<U>
//     where F: FnMut(Self::Item<'a>) -> U;
//
// Implement it for `&[T]` such that:
//   - Item<'a> = &'a T
//   - Result<U> = Vec<U>
// i.e., mapping a slice by reference produces a `Vec<U>` (no allocations per item,
// just the final Vec).

// TODO Write your trait and its slice impl.

// === Tests ===================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_by_ref_mapping() {
        // Requires Part C
        let s: &[i32] = &[1, 2, 3];
        assert_eq!(s.map_ref(|x| x + 1), vec![2, 3, 4]);
        let s2: &[&str] = &["a", "bb", "ccc"];
        assert_eq!(s2.map_ref(|x| x.len()), vec![1, 2, 3]);
    }

    #[test]
    fn slice_by_ref_mapping_struct_borrows() {
        // A non-Copy type where moving/cloning would be undesirable.
        struct User {
            name: String,
            id: u32,
        }

        let users = [
            User { name: "Alice".into(), id: 1 },
            User { name: "Bob".into(), id: 2 },
            User { name: "Carol".into(), id: 3 },
        ];

        let s = &users[..];
        // Borrowed output: Vec<&str>
        let names = s.map_ref(|u| u.name.as_str());
        assert_eq!(names, vec!["Alice", "Bob", "Carol"]);

        // Owned output: Vec<String>, built from borrowed inputs
        let ids = s.map_ref(|u| u.id.to_string());
        assert_eq!(ids, vec!["1", "2", "3"]);

        struct User2 {
            name: String,
            id: u32,
            active: bool,
        }

        let users2 = s.map_ref(|u| User2 {
            name: u.name.clone(),
            id: u.id,
            active: true,
        });

        let s2 = users2.as_slice();
        let names2 = s2.map_ref(|u| u.name.as_str());
        assert_eq!(names2, vec!["Alice", "Bob", "Carol"]);

        // Original data is intact (no moves)
        assert_eq!(users[0].name, "Alice");
    }
}