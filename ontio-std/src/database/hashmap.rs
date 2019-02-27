//use alloc::collections::BTreeMap;
//use super::list::List;
//use crate::Vec;
//
//pub struct HashMap {
//    key: String,
//    size: u32,
//    cache: BTreeMap<T, T>,
//    key_list: List<T>,
//    remove_list: Vec<T>,//store all removed key
//}
//
//impl Drop for HashMap
//{
//    fn drop(&mut self) {
//        self.flush();
//    }
//}
//
//impl HashMap {
//    pub fn new(key:String)  -> HashMap {
//        let cache:BTreeMap<T,T> = BTreeMap::new();
//        let list = List::new(key.clone());
//        HashMap {
//            key:key.clone(),
//            size:0,
//            cache:cache,
//            key_list:List::new(key),
//            remove_list: Vec::new(),
//        }
//    }
//    pub fn open(key: String) -> HashMap {
//        HashMap::new(key)
//    }
//    pub fn put(&mut self, key: T, value: T) {
//
//    }
//    pub fn flush(&mut self) {
//    }
//}