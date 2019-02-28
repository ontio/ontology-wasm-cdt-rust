use super::abi::{Decoder, Encoder};
use super::database;
use super::list::List;
use crate::Vec;
use alloc::collections::BTreeMap;
use std::clone::Clone;
use std::cmp::Ord;
use std::convert::AsRef;

pub struct HashMap<K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    size: u32, //hashmap size
    cache: BTreeMap<K, V>,
    key_list: List<K>,   //store all the key
    remove_list: Vec<K>, //store all removed key
    need_flush: Vec<K>,  //store all need flush key
}

impl<K, V> Drop for HashMap<K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    fn drop(&mut self) {
        self.flush();
    }
}

impl<K, V> HashMap<K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    pub fn new(key: String) -> HashMap<K, V> {
        let cache: BTreeMap<K, V> = BTreeMap::new();
        HashMap {
            size: 0,
            cache: cache,
            key_list: List::new(key),
            remove_list: Vec::new(),
            need_flush: Vec::new(),
        }
    }
    pub fn open(key: String) -> HashMap<K, V> {
        let mut hashmap = HashMap::new(key.clone());
        let list: List<K> = List::open(key);
        hashmap.size = list.len();
        hashmap.key_list = list;
        hashmap
    }
    pub fn put(&mut self, key: K, value: V) {
        //push_key==true means database does not have the key
        //key can not be same in key list
        let mut push_key = true;
        if self.cache.contains_key(&key) {
            push_key = false;
        } else {
            if let Some(_) = database::get::<_, V>(&key) {
                push_key = false;
            }
        }
        self.cache.insert(key.clone(), value);
        self.size += 1;
        self.need_flush.push(key.clone());
        if push_key {
            self.key_list.push(key);
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

    pub fn remove(&mut self, key: &K) {
        if self.size < 1 {
            panic!("null hashmap");
        }
        //if key in cache
        if self.cache.contains_key(key) {
            self.cache.remove(key);
        }
        database::delete(key);
        //store all remove key, when flush, will update key list
        if !self.remove_list.contains(key) {
            self.remove_list.push(key.clone());
            self.size -= 1;
        }
    }
    pub fn flush(&mut self) {
        //store cache data to database
        for k in &self.need_flush {
            if let Some(v) = self.cache.get(k) {
                database::put(k, v);
            }
        }
        let size = self.size;
        let mut remove_size = self.remove_list.len();
        //all removed key store in remove_list
        for ind in 0..size {
            if let Some(data) = self.key_list.get(ind) {
                if remove_size > 0 {
                    if self.remove_list.contains(data) {
                        self.key_list.remove(ind);
                        remove_size -= 1;
                    }
                } else {
                    break;
                }
            }
        }
        //        let mut remove_index = Vec::new();
        //        {
        //            let iter = &mut self.key_list.iter();
        //            while let Some(key) = iter.next() {
        //                if self.remove_list.contains(key) {
        //                    self.remove_list.remove(index as usize);
        //                    remove_index.push(index);
        //                }
        //                index += 1;
        //            }
        //        }
        //        for index in remove_index {
        //            self.key_list.remove(index);
        //        }
        self.remove_list.clear();
        self.key_list.flush();
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
    map: &'a mut HashMap<K, V>,
}

impl<'a, K, V> Iterator<'a, K, V>
where
    K: AsRef<[u8]> + Encoder + Decoder + Ord + Clone,
    V: Encoder + Decoder + Clone,
{
    fn new(cursor: u32, map: &mut HashMap<K, V>) -> Iterator<K, V> {
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
    let mut m = HashMap::new("test".to_string());
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
    let mut m = HashMap::new("test".to_string());
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

    let mut m2: HashMap<String, String> = HashMap::open("test".to_string());
    assert_eq!(m2.size, 90);
    assert_eq!(m2.get(&"hello0".to_string()).unwrap(), "world0");

    m2.remove(&"hello0".to_string());
    assert_eq!(m2.size, 89);
    assert_eq!(m2.get(&"hello0".to_string()), None);
}
