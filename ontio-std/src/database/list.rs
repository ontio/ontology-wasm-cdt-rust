use crate::abi::{Decoder, Encoder, Error, Sink, Source};
use crate::cmp::PartialEq;
use crate::database;
use crate::{format, String, Vec};
use alloc::collections::BTreeMap;

//slice default size
const INDEX_SIZE: u32 = 64;

pub struct ListStore<T>
where
    T: Encoder + Decoder,
{
    key: String,
    need_flush: Vec<u32>, //index,store all index which slice need update
    size: u32,
    next_key_id: u32,
    index_size: Vec<(u32, u32)>,  //index, count
    cache: BTreeMap<u32, Vec<T>>, //index, vec
}

impl<T> Drop for ListStore<T>
where
    T: Encoder + Decoder,
{
    fn drop(&mut self) {
        self.flush();
    }
}

impl<T> ListStore<T>
where
    T: Encoder + Decoder + PartialEq,
{
    pub(crate) fn contains(&mut self, value: &T) -> bool {
        if self.size <= 0 {
            return false;
        } else {
            for i in 0..self.size {
                if let Some(data) = self.get(i) {
                    if data == value {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl<T> ListStore<T>
where
    T: Encoder + Decoder,
{
    fn encode(&self, sink: &mut Sink) {
        sink.write(self.next_key_id);
        sink.write(&self.index_size);
    }

    fn init(key: String, source: &mut Source) -> Result<Self, Error> {
        let next_key_id = source.read().unwrap();
        let index_size: Vec<(u32, u32)> = source.read().unwrap();
        let total = index_size.iter().map(|(_key, size)| size).sum();
        Ok(ListStore {
            key: key,
            need_flush: Vec::new(), //index,store all index which slice need update
            size: total,
            next_key_id: next_key_id,
            index_size: index_size, //index, count
            cache: BTreeMap::new(), //index, vec
        })
    }

    pub(crate) fn new(key: String) -> ListStore<T> {
        let need_flush: Vec<u32> = Vec::default();
        let index_count: Vec<(u32, u32)> = Vec::new();
        let cache: BTreeMap<u32, Vec<T>> = BTreeMap::new();
        ListStore {
            key: key,
            need_flush: need_flush,
            size: 0,
            next_key_id: 0,
            index_size: index_count,
            cache: cache,
        }
    }
    pub fn open(key: String) -> ListStore<T> {
        match database::get(&key) {
            None => ListStore::new(key),
            Some(data) => {
                let mut source = Source::new(data);
                ListStore::init(key, &mut source).unwrap()
            }
        }
    }
    pub fn len(&self) -> u32 {
        self.size
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            Some(self.remove(self.size - 1))
        }
    }

    pub fn remove(&mut self, index: u32) -> T {
        if index >= self.size {
            panic!("[remove] index out of bound");
        } else {
            let mut end = 0;
            let ind = self
                .index_size
                .iter()
                .take_while(|&x| {
                    end += x.1;
                    index >= end
                })
                .count();
            let mut bulk = &mut self.index_size[ind];
            let start = end - bulk.1;
            //if data in cache
            let val = if let Some(x) = self.cache.get_mut(&bulk.0) {
                x.remove((index - start) as usize)
            } else {
                //read data from database
                let key = format!("{}{}", self.key, bulk.0);
                let data = database::get(key).unwrap();
                let mut source = Source::new(data);
                let l = source.read_u32().unwrap();
                let mut temp: Vec<T> = Vec::new();
                for _ in 0..l {
                    temp.push(source.read().unwrap());
                }
                let val = temp.remove((index - start) as usize);
                self.cache.insert(bulk.0, temp);
                val
            };
            //update need_flush
            if !self.need_flush.contains(&bulk.0) {
                self.need_flush.push(bulk.0);
            }
            //update index_size
            bulk.1 = bulk.1 - 1;
            //update list size
            self.size = self.size - 1;
            val
        }
    }

    pub fn push(&mut self, payload: T) {
        //if null list
        if self.index_size.is_empty() {
            //update cache
            let mut temp: Vec<T> = Vec::new();
            temp.push(payload);
            self.cache.insert(0, temp);
            //update index_count
            self.index_size.push((0, 1));
            self.need_flush.push(0);
            self.next_key_id += 1;
        } else {
            //update the last index_count and the last key->data
            let mut last_index_count = self.index_size.last_mut().unwrap();
            //if data not in cache
            if !self.cache.contains_key(&last_index_count.0) {
                //read data from database
                let keyn = format!("{}{}", self.key, last_index_count.0);
                let last_node_vec_data = database::get(keyn).unwrap();
                let mut source = Source::new(last_node_vec_data);
                let last_length = source.read_u32().unwrap();
                let mut last_node_vec: Vec<T> = Vec::new();
                for _ in 0..last_length {
                    last_node_vec.push(source.read().unwrap());
                }
                self.cache.insert(last_index_count.0, last_node_vec);
            }
            //if data in cache
            if let Some(last_node_vec) = self.cache.get_mut(&last_index_count.0) {
                //if the slice is filled
                let l = last_node_vec.len() as u32;
                if l >= INDEX_SIZE {
                    let mut temp: Vec<T> = Vec::new();
                    temp.push(payload);
                    //cache add new k->v
                    self.cache.insert(self.next_key_id, temp);
                    self.index_size.push((self.next_key_id, 1));
                    self.need_flush.push(self.next_key_id);
                    self.next_key_id = self.next_key_id + 1;
                } else {
                    //if slice is not filled
                    last_node_vec.push(payload);
                    //update need_flush
                    if !self.need_flush.contains(&last_index_count.0) {
                        self.need_flush.push(last_index_count.0);
                    }
                    //update index_count
                    last_index_count.1 = last_index_count.1 + 1
                }
            }
        }
        self.size = self.size + 1;
    }

    pub fn insert(&mut self, index: u32, payload: T) {
        if index >= self.size {
            panic!("index out of bound");
        } else {
            let end = &mut 0;
            let ind = self
                .index_size
                .iter()
                .take_while(|&x| {
                    *end += x.1;
                    *end <= index
                })
                .count();
            let bulk = &mut self.index_size[ind];
            let start = *end - bulk.1;
            if self.cache.contains_key(&bulk.0) {
                //if data in cache
                let temp = self.cache.get_mut(&bulk.0).unwrap();
                temp.insert((index - start) as usize, payload);
            } else {
                //read data from db
                let key = format!("{}{}", self.key, bulk.0);
                match database::get(&key) {
                    Some(data) => {
                        let mut source = Source::new(data);
                        let l = source.read_u32().unwrap();
                        let mut temp: Vec<T> = Vec::new();
                        for _ in 0..l {
                            temp.push(source.read().unwrap());
                        }
                        temp.insert((index - start) as usize, payload);
                        self.cache.insert(index, temp);
                    }
                    None => unreachable!(),
                }
            }
            bulk.1 = bulk.1 + 1;
            self.size = self.size + 1;
        }
    }

    pub fn clear(&mut self) {
        let index_size = self.index_size.to_vec();
        for bulk in index_size {
            let key = format!("{}{}", self.key, bulk.0);
            database::delete(&key);
        }
        self.need_flush.clear();
        self.next_key_id = 0;
        self.index_size.clear();
        self.size = 0;
        self.cache.clear();
    }

    pub fn flush(&mut self) {
        if !self.need_flush.is_empty() {
            let need_flush = self.need_flush.to_vec();
            for k in need_flush {
                let v = self.cache.get(&k).unwrap();
                let l = v.len() as u32;
                let mut sink = Sink::new(16);
                sink.write_u32(l);
                for i in v {
                    i.encode(&mut sink);
                }
                let key = format!("{}{}", self.key, k);
                database::put(&key, sink.bytes());
            }
            let mut sink = Sink::new(16);
            self.encode(&mut sink);
            database::put(&self.key, sink.bytes())
        }
    }

    pub fn iter(&mut self) -> Iterator<T> {
        Iterator::new(0, self)
    }

    pub fn get(&mut self, index: u32) -> Option<&T> {
        if index >= self.size {
            panic!("[get] index out of bound")
        }
        let mut end = 0;
        let ind = self
            .index_size
            .iter()
            .take_while(|&x| {
                end = end + x.1;
                end <= index
            })
            .count();
        let bulk = &self.index_size[ind];
        let start = end - bulk.1;
        //if data not in cache, read data from database
        if self.cache.get(&bulk.0).is_none() {
            let key = format!("{}{}", self.key, bulk.0);
            let data = database::get(key).unwrap();
            let mut source = Source::new(data);
            let l = source.read_u32().unwrap();
            let mut temp: Vec<T> = Vec::new();
            for _ in 0..l {
                temp.push(source.read().unwrap());
            }
            self.cache.insert(bulk.0, temp);
        }
        return self.cache.get(&bulk.0).unwrap().get((index - start) as usize);
    }
}

pub struct Iterator<'a, T>
where
    T: Encoder + Decoder,
{
    cursor: u32,
    list: &'a mut ListStore<T>,
}

impl<'a, T> Iterator<'a, T>
where
    T: Encoder + Decoder,
{
    fn new(cursor: u32, list: &mut ListStore<T>) -> Iterator<T> {
        Iterator { cursor: cursor, list: list }
    }

    pub fn has_next(&self) -> bool {
        if self.cursor >= self.list.size {
            return false;
        }
        true
    }

    pub fn next(&mut self) -> Option<&T> {
        if !self.has_next() {
            return None;
        }
        let temp = self.list.get(self.cursor);
        self.cursor = self.cursor + 1;
        temp
    }
}

#[test]
fn test_insert() {
    let mut list: ListStore<String> = ListStore::new("key".to_string());
    for x in 0..90 {
        list.push(format!("hello{}", x));
    }
    list.insert(64, "hello90".to_string());
    assert_eq!(list.size, 91);
    assert_eq!(list.get(0).unwrap(), "hello0");
    assert_eq!(list.get(1).unwrap(), "hello1");
    assert_eq!(list.get(2).unwrap(), "hello2");
    assert_eq!(list.get(64).unwrap(), "hello90");

    list.remove(64);
    assert_eq!(list.size, 90);
    assert_eq!(list.get(64).unwrap(), "hello64");
}

#[test]
fn list_node() {
    let mut list: ListStore<String> = ListStore::new("key".to_string());
    list.push("value".to_string());
    list.push("sss".to_string());
    //    list.append(123);
    assert_eq!(list.size, 2);
    if let Some(x) = list.get(1) {
        assert_eq!(x, "sss")
    }
    list.flush();
    let mut list2: ListStore<String> = ListStore::open("key".to_string());
    assert_eq!(list2.size, 2);

    list2.remove(1);
    assert_eq!(list2.size, 1);
    assert_eq!(list2.need_flush.len(), 1);
    list2.flush();

    let list3: ListStore<String> = ListStore::open("key".to_string());
    assert_eq!(list3.size, 1);
}

#[test]
fn test_iter() {
    let mut list: ListStore<String> = ListStore::new("key".to_string());
    for x in 0..90 {
        list.push(format!("hello{}", x));
    }
    let iter = &mut list.iter();
    let mut i = 0;
    loop {
        if iter.has_next() {
            if let Some(data) = iter.next() {
                assert_eq!(format!("hello{}", i).as_str(), data);
            } else {
                assert_eq!(i, 90);
            }
            i += 1;
        } else {
            break;
        }
    }
}

#[test]
fn clear() {
    let mut list: ListStore<String> = ListStore::open("key".to_string());
    for x in 0..90 {
        list.push(format!("hello{}", x));
    }
    assert_eq!(list.contains(&format!("hello{}", 0)), true);
    assert_eq!(list.size, 90);
    list.clear();
    assert_eq!(list.size, 0);
}
#[test]
fn mock_test() {
    for _n in 0..1000 {
        let mut list: ListStore<u64> = ListStore::new("key".to_string());
        let mut array = Vec::new();
        for _i in 0..100 {
            match rand::random::<u8>() {
                0..50 => {
                    let val = rand::random();
                    list.push(val);
                    array.push(val);
                }
                51..100 => {
                    if array.len() != 0 {
                        let val = rand::random();
                        let pos = rand::random::<usize>() % array.len();
                        list.insert(pos as u32, val);
                        array.insert(pos, val);
                    }
                }
                101..150 => {
                    if array.len() != 0 {
                        let pos = rand::random::<usize>() % array.len();
                        let val = list.get(pos as u32);
                        assert_eq!(val, array.get(pos as usize));
                    }
                }
                151..180 => {
                    if array.len() != 0 {
                        let pos = rand::random::<usize>() % array.len();
                        list.remove(pos as u32);
                        array.remove(pos);
                    }
                }
                181..200 => {
                    let mut temp = Vec::new();
                    let mut iter = list.iter();
                    while let Some(val) = iter.next() {
                        temp.push(*val);
                    }
                    assert_eq!(temp, array);
                }
                _ => (),
            }
        }
    }
}
