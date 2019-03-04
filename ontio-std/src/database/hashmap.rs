use super::list::ListStore;
use crate::abi::{Decoder, Encoder};
use crate::database;
use crate::prelude::*;
use crate::Vec;
use alloc::collections::BTreeMap;
use alloc::prelude::String;

pub struct HashMapStore<K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    size: u32, //hashmap size
    cache: BTreeMap<K, V>,
    key_list: ListStore<K>,   //store all the key
    remove_list: Vec<K>, //store all removed key
    need_flush: Vec<K>,  //store all need flush key
}

impl<K, V> Drop for HashMapStore<K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    fn drop(&mut self) {
        self.flush();
    }
}

impl<K, V> HashMapStore<K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    pub(crate) fn new(key: String) -> HashMapStore<K, V> {
        let cache: BTreeMap<K, V> = BTreeMap::new();
        HashMapStore {
            size: 0,
            cache: cache,
            key_list: ListStore::new(key),
            remove_list: Vec::new(),
            need_flush: Vec::new(),
        }
    }
    pub fn open(key: String) -> HashMapStore<K, V> {
        let mut hashmap = HashMapStore::new(key.clone());
        let list: ListStore<K> = ListStore::open(key);
        hashmap.size = list.len();
        hashmap.key_list = list;
        hashmap
    }
    pub fn put(&mut self, key: K, value: V) {
        //push_key==true means database does not have the key
        //key can not be same in key list
        if !self.contains_key(&key) {
            self.key_list.push(key.clone());
            self.size += 1;
        }
        if self.remove_list.contains(&key) {
            let ind = self.remove_list.iter().take_while(|x| {
                *x != &key
            }).count();
            self.remove_list.remove(ind);
        }
        self.cache.insert(key.clone(), value);
        if !self.need_flush.contains(&key) {
            self.need_flush.push(key);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if !self.cache.contains_key(key) {
            if let Some(value) = database::get(key) {
                self.cache.insert(key.clone(), value);
            } else {
                return None;
            }
        }
        self.cache.get(&key)
    }
    fn contains_key(&mut self, key:&K) -> bool {
        if self.cache.contains_key(key) {
            return true;
        }
        if self.remove_list.contains(key) {
            return false;
        } else {
            if let Some(_data) = database::get::<_,V>(key) {
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, key: &K) {
        if self.size < 1 {
            return;
        }
        //store all remove key, when flush, will update key list
        if self.contains_key(key) {
            if !self.remove_list.contains(key) {
                self.remove_list.push(key.clone());
                self.size -= 1;
            }
            database::delete(key);
            //if key in cache
            if self.cache.contains_key(key) {
                self.cache.remove(key);
            }
        }
    }
    pub fn flush(&mut self) {
        //store cache data to database
        for k in &self.need_flush {
            if let Some(v) = self.cache.get(k) {
                database::put(k, v);
            }
        }
        let remove_size = self.remove_list.len() as u32;
        let key_list_length = self.key_list.len();
        let mut has_removed_num:u32 = 0;
        //all removed key store in remove_list
        for ind in 0..key_list_length {
            if let Some(data) = self.key_list.get(ind-has_removed_num) {
                if remove_size > has_removed_num {
                    if self.remove_list.contains(data) {
                        self.key_list.remove(ind - has_removed_num);
                        has_removed_num += 1;
                    }
                } else {
                    break;
                }
            }
        }
        self.remove_list.clear();
        self.key_list.flush();
    }

    pub fn clear(&mut self) {
        let size = self.key_list.len();
        for i in 0..size {
            if let Some(key) = self.key_list.get(i) {
                database::delete(key);
            }
        }
        self.key_list.clear();
        self.need_flush.clear();
        self.cache.clear();
        self.remove_list.clear();
        self.size = 0;
    }

    pub fn iter(&mut self) -> Iterator<K, V> {
        Iterator::new(0, self)
    }
}

pub struct Iterator<'a, K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    cursor: u32,
    map: &'a mut HashMapStore<K, V>,
}

impl<'a, K, V> Iterator<'a, K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    fn new(cursor: u32, map: &mut HashMapStore<K, V>) -> Iterator<K, V> {
        Iterator { cursor: cursor, map: map }
    }
    pub fn has_next(&self) -> bool {
        if self.map.size > self.cursor {
            true
        } else {
            false
        }
    }
    pub fn next(&mut self) -> Option<(&K, V)> {
        if self.cursor >= self.map.size {
            None
        } else {
            let list = &mut self.map.key_list;
            let key = list.get(self.cursor).unwrap();
            let v = {
                if !self.map.cache.contains_key(key) {
                    if let Some(value) = database::get(key) {
                        self.map.cache.insert(key.clone(), value);
                    }
                }
                self.map.cache.get(&key)
            };
            self.cursor += 1;
            if let Some(value) = v {
                Some((key, value.clone()))
            } else {
                None
            }
        }
    }
}

#[test]
fn get() {
    let mut m: HashMapStore<String, String> = HashMapStore::new("test".to_string());
    for i in 0..90 {
        m.put(format!("hello{}", i), format!("world{}", i));
    }
    assert_eq!(m.get(&"hello1".to_string()).unwrap(), "world1");

    assert_eq!(m.size, 90);
    assert_eq!(m.get(&"hello0".to_string()).unwrap(), "world0");
    m.remove(&"hello0".to_string());
    assert_eq!(m.get(&"hello0".to_string()).is_some(), false);
}

#[test]
fn iter() {
    let mut m = HashMapStore::new("test".to_string());
    for i in 0..90 {
        m.put(format!("hello{}", i), format!("world{}", i));
    }

    let mut iter = m.iter();
    let mut ind = 0;
    while let Some((k, v)) = iter.next() {
        assert_eq!(k, &format!("hello{}", ind));
        assert_eq!(v, format!("world{}", ind));
        ind += 1;
    }

    m.flush();

    let mut m2: HashMapStore<String, String> = HashMapStore::open("test".to_string());
    assert_eq!(m2.size, 90);
    assert_eq!(m2.get(&"hello0".to_string()).unwrap(), "world0");

    m2.remove(&"hello0".to_string());
    assert_eq!(m2.size, 89);
    assert_eq!(m2.get(&"hello0".to_string()), None);
}

#[test]
fn remove() {
    let mut m = HashMapStore::new("test".to_string());
    for i in 0..90 {
        m.put(format!("hello{}", i), format!("world{}", i));
    }
    assert_eq!(m.size, 90);

    for x in 0..30 {
        m.remove(&format!("hello{}", x));
    }
    assert_eq!(m.size, 60);

    for x in 0..30 {
        assert_eq!(m.get(&format!("hello{}", x)), None);
    }
}
#[test]
fn iter_remove() {
    let mut m = HashMapStore::new("test".to_string());
    for i in 0..10 {
        m.put(format!("hello{}", i), format!("world{}", i));
    }
    assert_eq!(m.size, 10);
    for i in 0..10 {
        m.remove(&format!("hello{}", i));
    }
    assert_eq!(m.size, 0);
}

#[test]
fn mock_test() {
    for _n in 0..1000 {
        let mut map: HashMapStore<String, u32> = HashMapStore::open("key".to_string());
        map.clear();
        let mut bmap:BTreeMap<String, u32> = BTreeMap::new();
        for _i in 0..1000 {
            match rand::random::<u8>()%150 {
                0..50 => {
                    assert_eq!(map.size, bmap.len() as u32);
                    let val = rand::random::<u32>() % 50;
                    map.put(format!("{}", val), val);
                    bmap.insert(format!("{}", val), val);
                }
                51..100 => {
                    if bmap.len() != 0 {
                        assert_eq!(map.size, bmap.len() as u32);
                        let pos = rand::random::<usize>() % bmap.len();
                        let val = map.get(&format!("{}", pos));
                        assert_eq!(val, bmap.get(&format!("{}", pos)));
                    }
                }
                101..150 => {
                    if bmap.len() != 0 {
                        assert_eq!(map.size, bmap.len() as u32);
                        let pos = rand::random::<usize>() % bmap.len();
                        map.remove(&format!("{}", pos));
                        bmap.remove(&format!("{}", pos));
                    }
                }
                _ => (),
            }
        }
    }
}
