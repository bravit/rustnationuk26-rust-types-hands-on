// Trait Bounds & Generic Constraints
//
// Complete the TODOs so the tests pass.
// Try to figure out the types and constraints used for each exercise.

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io::{Read, Seek};
use std::path::Path;

// -----------------------------
// Domain types used across exercises
// -----------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Vec2(pub f32, pub f32);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum GameEvent {
    Damage { target: EntityId, amount: u32 },
    Heal { target: EntityId, amount: u32 },
    Spawn(EntityId),
}

// -----------------------------
// Exercise 1: Generic resource loader
// -----------------------------
// Goal: implement a loader that can read from either a path-like or any reader/seek source.
// Constraints to practice: `AsRef<Path>`, `Read + Seek`, `Display` for error context.

#[derive(Debug, PartialEq)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid header: {0}")]
    InvalidHeader(String),
}

/// TODO: Implement this function to load a texture from any type that implements Read + Seek.
/// Add a `where` clause constraining R: Read + Seek + Debug (so tests can print it on failure).
pub fn load_texture_from_reader<R>(_reader: &mut R) -> Result<Texture, LoadError>
{
    // For the exercise we won't implement real parsing; just simulate reading.
    // TODO: Use the reader to read at least 8 bytes; return a Texture(64, 64) if ok, else error.
    // If fewer than 8 bytes, return LoadError::InvalidHeader("too short").
    let _ = _reader; // remove when implemented
    Err(LoadError::InvalidHeader("unimplemented".into()))
}

/// TODO: Implement a convenience wrapper that accepts any AsRef<Path> and opens the file.
pub fn load_texture_from_path<P>(_path: P) -> Result<Texture, LoadError>
{
    let _ = _path; // remove when implemented
    Err(LoadError::InvalidHeader("unimplemented".into()))
}

// -----------------------------
// Exercise 2: Systems with trait bounds & associated types
// -----------------------------
// Goal: Define traits PositionLike and VelocityLike with an associated scalar type `Scalar` and constraints on it.
// Implement `advance` that moves a position by velocity * dt.
// Practice: associated type bounds + where-clauses with ops.

use std::ops::{Add, Mul};

pub trait PositionLike: Clone {
    type Scalar: Copy + Mul<Output = Self::Scalar> + Add<Output = Self::Scalar> + From<f32> + PartialEq + Debug;
    fn xy(&self) -> (Self::Scalar, Self::Scalar);
    fn from_xy(x: Self::Scalar, y: Self::Scalar) -> Self;
}

pub trait VelocityLike: Clone {
    type Scalar: Copy + Mul<Output = Self::Scalar> + Add<Output = Self::Scalar> + From<f32> + PartialEq + Debug;
    fn xy(&self) -> (Self::Scalar, Self::Scalar);
}

impl PositionLike for Vec2 {
    type Scalar = f32;
    fn xy(&self) -> (Self::Scalar, Self::Scalar) { (self.0, self.1) }
    fn from_xy(x: Self::Scalar, y: Self::Scalar) -> Self { Vec2(x, y) }
}
impl VelocityLike for Vec2 {
    type Scalar = f32;
    fn xy(&self) -> (Self::Scalar, Self::Scalar) { (self.0, self.1) }
}

/// TODO: Implement `advance(p, v, dt)` generically for any P: PositionLike and V: VelocityLike with the SAME Scalar.
/// Use a where-clause to relate `P::Scalar == V::Scalar` and require the ops needed.
pub fn advance<P, V>(_p: &P, _v: &V, _dt: P::Scalar) -> P
where
    P: PositionLike,
    V: VelocityLike,
{
    let _ = (_p, _v, _dt);
    unimplemented!("advance")
}

// -----------------------------
// Exercise 3: Event aggregation over iterators with Borrow and Into
// -----------------------------
// Goal: Sum damage per entity over any iterator whose items borrow `GameEvent`.
// Practice: `Iterator`, `Borrow<T>`, `Into` for returning a map, `FromIterator` maybe.

/// TODO: Implement `damage_totals` so it accepts ANY iterator of items that `Borrow<GameEvent>`.
/// Return a HashMap<EntityId, u32> of total damage.
pub fn damage_totals<I, E>(events: I) -> HashMap<EntityId, u32>
where
    I: IntoIterator<Item = E>,
    E: Borrow<GameEvent>,
{
    let mut totals = HashMap::new();
    for _e in events {
        // TODO: use e.borrow() and match on GameEvent::Damage
    }
    totals
}

// -----------------------------
// Exercise 4: Generic cache with policy trait and trait-object alternative
// -----------------------------
// Goal: Implement a cache with bounds over K and V; add a policy trait controlling admission.
// Practice: Eq + Hash for keys, Clone for V, trait object `dyn CapacityPolicy<K, V> + Send + Sync`.

pub trait CapacityPolicy<K, V>: Send + Sync {
    fn admit(&self, key: &K, value: &V) -> bool;
}

pub struct Cache<K, V> {
    map: HashMap<K, V>,
    policy: Box<dyn CapacityPolicy<K, V>>,
    max: usize,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    /// TODO: enforce that `max > 0` and `policy.admit` before insert; evict oldest arbitrary item when full.
    pub fn new(policy: Box<dyn CapacityPolicy<K, V>>, max: usize) -> Self { Self { map: HashMap::new(), policy, max } }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        // TODO: bounds are already on impl; use `self.policy.admit(&key, &value)`
        let _ = (&key, &value);
        false
    }

    pub fn get(&self, key: &K) -> Option<&V> { self.map.get(key) }

    pub fn len(&self) -> usize { self.map.len() }
}

// A simple policy: admit only values whose Display is shorter than a threshold.
#[derive(Debug)]
pub struct DisplayLenPolicy<T> { pub max_chars: usize, _phantom: std::marker::PhantomData<T> }
impl<T> DisplayLenPolicy<T> { pub fn new(max_chars: usize) -> Self { Self { max_chars, _phantom: Default::default() } } }

impl<K, V> CapacityPolicy<K, V> for DisplayLenPolicy<V>
where
    V: Display + Send + Sync + 'static,
{
    fn admit(&self, _key: &K, value: &V) -> bool { value.to_string().chars().count() <= self.max_chars }
}

// -----------------------------
// Exercise 5: Error types and trait bounds on associated types
// -----------------------------
// Goal: a storage trait whose associated Error must be a real error type usable in anyhow/eyre.
// Practice: `type Error: std::error::Error + Send + Sync + 'static`.

pub trait Storage {
    type Error: std::error::Error + Send + Sync + 'static;
    fn save(&mut self, id: &str, bytes: &[u8]) -> Result<(), Self::Error>;
    fn load(&mut self, id: &str) -> Result<Vec<u8>, Self::Error>;
}

#[derive(Default)]
pub struct MemoryStore { map: HashMap<String, Vec<u8>> }

#[derive(Debug, thiserror::Error)]
pub enum MemErr { #[error("missing: {0}")] Missing(String) }

impl Storage for MemoryStore {
    type Error = MemErr;
    fn save(&mut self, id: &str, bytes: &[u8]) -> Result<(), Self::Error> {
        // TODO
        Ok(())
    }
    fn load(&mut self, id: &str) -> Result<Vec<u8>, Self::Error> {
        // TODO
        Ok(vec![])
    }
}

// -----------------------------
// Exercise 6: Higher-level constraints with Fn and Send + 'static
// -----------------------------
// Goal: register a callback that can be run on a background thread; must be `Send + 'static`.
// Practice: bounds on function type parameter.

pub struct Dispatcher { listeners: Vec<Box<dyn Fn(GameEvent) + Send + Sync + 'static>> }
impl Dispatcher {
    pub fn new() -> Self { Self { listeners: vec![] } }

    /// TODO: accept any F that can be converted into a boxed listener with bounds: Fn(GameEvent) + Send + Sync + 'static.
    pub fn on<E, F>(&mut self, _filter: E, _f: F)
    where
        E: Borrow<HashSet<EntityId>>,
        F: Fn(GameEvent) + Send + Sync + 'static,
    {
        let _ = _filter; let _ = _f; // remove
    }

    pub fn dispatch(&self, ev: GameEvent) { for f in &self.listeners { f(ev.clone()) } }
}

// -----------------------------
// Tests (failing until you implement the TODOs)
// -----------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Cursor, Seek, SeekFrom, Write};
    use std::sync::{Arc, Mutex};

    // --- Exercise 1 ---
    #[test]
    fn load_from_reader_min_bytes() {
        let mut buf = Cursor::new(vec![0u8; 8]);
        let tex = load_texture_from_reader(&mut buf).expect("should load");
        assert_eq!(tex.width, 64);
        assert_eq!(tex.height, 64);
    }

    #[test]
    fn load_from_reader_too_short() {
        let mut buf = Cursor::new(vec![0u8; 7]);
        let err = load_texture_from_reader(&mut buf).unwrap_err();
        assert!(format!("{err}").contains("too short"));
    }

    #[test]
    fn load_from_path() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("tex.bin");
        let mut f = File::create(&p).unwrap();
        f.write_all(&[0u8; 8]).unwrap();
        drop(f);
        let tex = load_texture_from_path(&p).unwrap();
        assert_eq!(tex, Texture { width: 64, height: 64 });
    }

    // --- Exercise 2 ---
    #[test]
    fn advance_vec2() {
        let p = Vec2(1.0, 2.0);
        let v = Vec2(3.0, 4.0);
        let out = advance(&p, &v, 0.5);
        assert_eq!(out, Vec2(1.0 + 3.0*0.5, 2.0 + 4.0*0.5));
    }

    // --- Exercise 3 ---
    #[test]
    fn aggregate_damage_from_borrowed_items() {
        let evs = vec![
            GameEvent::Spawn(EntityId(1)),
            GameEvent::Damage { target: EntityId(1), amount: 10 },
            GameEvent::Damage { target: EntityId(2), amount: 5 },
            GameEvent::Damage { target: EntityId(1), amount: 7 },
        ];
        let map = damage_totals(&evs); // iterator of &GameEvent items via Borrow
        assert_eq!(map.get(&EntityId(1)).copied(), Some(17));
        assert_eq!(map.get(&EntityId(2)).copied(), Some(5));
    }

    // --- Exercise 4 ---
    #[test]
    fn cache_with_policy_and_bounds() {
        let policy: Box<dyn CapacityPolicy<String, String>> = Box::new(DisplayLenPolicy::<String>::new(8));
        let mut c: Cache<String, String> = Cache::new(policy, 2);
        assert!(!c.insert("a".into(), "this is longer than 8".into())); // rejected by policy
        assert!(c.insert("a".into(), "short".into()));
        assert!(c.insert("b".into(), "small".into()));
        assert_eq!(c.len(), 2);
        // cache full: inserting a third admitted item should evict one existing entry
        assert!(c.insert("c".into(), "tiny".into()));
        assert_eq!(c.len(), 2);
        // At least the newly inserted one exists
        assert!(c.get(&"c".into()).is_some());
    }

    // --- Exercise 5 ---
    #[test]
    fn storage_trait_bounds() {
        let mut s = MemoryStore::default();
        s.save("k", b"hello").unwrap();
        let bytes = s.load("k").unwrap();
        assert_eq!(bytes, b"hello");
        let err = s.load("missing").unwrap_err();
        assert!(format!("{err}").contains("missing"));
    }

    // --- Exercise 6 ---
    #[test]
    fn dispatcher_accepts_send_static_closures() {
        let mut d = Dispatcher::new();
        let set: HashSet<EntityId> = [EntityId(1), EntityId(2)].into_iter().collect();

        // Use shared ownership + interior mutability
        let seen: Arc<Mutex<Vec<EntityId>>> = Arc::new(Mutex::new(Vec::new()));
        let seen_handle = Arc::clone(&seen);

        // move + Send + Sync + 'static closure that mutates via Mutex
        d.on(set.clone(), move |ev: GameEvent| {
            if let GameEvent::Spawn(id) = ev {
                seen_handle.lock().unwrap().push(id);
            }
        });

        d.dispatch(GameEvent::Spawn(EntityId(2)));

        assert_eq!(&*seen.lock().unwrap(), &vec![EntityId(2)]);
    }
}
