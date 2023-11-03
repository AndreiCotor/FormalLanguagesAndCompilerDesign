use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct MyHashMap<Key> {
    size: usize,
    table: Vec<Vec<Key>>
}

impl<Key: Eq + Hash + Clone + Debug> fmt::Display for MyHashMap<Key> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        let mut bucket = 0;
        for el in &self.table {
            res.push_str(&format!("Bucket {}: {:?}\n", bucket, el));
            bucket += 1;
        }

        write!(f, "{}", res)
    }
}

impl<Key: Eq + Hash + Clone> MyHashMap<Key> {
    pub fn new(size: usize) -> Self {
        let mut table = Vec::new();

        for _ in 0..size {
            table.push(Vec::new());
        }

        Self {
            size,
            table
        }
    }

    fn get_hash<Q>(&self, value: Q) -> usize
        where Q: Hash
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        return (hasher.finish() % (self.size as u64)) as usize;
    }

    pub fn insert(&mut self, key: Key) -> Result<(usize, usize), ()>
    {
        if self.get(&key).is_some() {
            return Err(());
        }

        let hash = self.get_hash(&key);

        self.table.get_mut(hash).expect("Hash function returned value larger than capacity")
            .push(key);

        Ok((hash, self.table.get(hash).expect("").len() - 1))
    }

    pub fn get<Q>(&self, key: &Q) -> Option<(usize, usize)>
        where Key: Borrow<Q>,
              Q: Hash + Eq + ?Sized
    {
        let hash = self.get_hash(&key);

        let list_opt = self.table.get(hash);
        if list_opt.is_none() {
            return None;
        }

        list_opt.unwrap().iter().position(|el| el.borrow() == key).map(|el| (hash, el))
    }

    pub fn get_by_position(&self, position: &(usize, usize)) -> Option<&Key> {
        match self.table.get(position.0) {
            Some(list) => list.get(position.1),
            None => None
        }
    }

    pub fn delete<Q>(&mut self, key: &Q) -> Result<(), ()>
        where Key: Borrow<Q>,
              Q: Hash + Eq + ?Sized
    {
        if self.get(key).is_none() {
            return Err(());
        }

        let hash = self.get_hash(&key);

        self.table.get_mut(hash)
            .expect("Hash function returned hash larger than size")
            .retain(|el| el.borrow() != key);

        return Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn before_each() -> MyHashMap<String> {
        let mut my_map = MyHashMap::new(16);

        my_map.insert("a".to_owned()).expect("");
        my_map.insert("b".to_owned()).expect("");
        my_map.insert("c".to_owned()).expect("");

        my_map
    }

    #[test]
    fn test_get() {
        let my_map = before_each();

        assert!(my_map.get("a").is_some());
        assert!(my_map.get("b").is_some());
        assert!(my_map.get("c").is_some());
        assert!(my_map.get("d").is_none());

        let val = my_map.get("a").expect("");
        assert_eq!(my_map.get_by_position(&val).unwrap(), "a");
    }

    #[test]
    fn test_insert() {
        let mut my_map = before_each();

        my_map.insert("d".to_owned()).expect("");
        assert!(my_map.get("d".borrow()).is_some());
        assert!(my_map.get("e").is_none());
    }

    #[test]
    fn test_delete() {
        let mut my_map = before_each();

        my_map.delete("c").expect("");
        assert!(my_map.get("c").is_none());
    }
}