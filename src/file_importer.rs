use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;
use smol_str::SmolStr;

static FILE_CACHE: Lazy<Mutex<HashMap<SmolStr, Arc<str>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn import_file(file: &Path) -> Arc<str> {
    Arc::clone(
        FILE_CACHE
            .lock()
            .unwrap()
            .entry(file.to_string_lossy().into())
            .or_insert_with(|| Arc::from(std::fs::read_to_string(file).unwrap())),
    )
}

pub fn register_input(name: &SmolStr, input: &str) -> Arc<str> {
    let mut cache = FILE_CACHE.lock().unwrap();
    cache.insert(name.to_owned(), Arc::from(input));
    Arc::clone(cache.get(name).unwrap())
}

pub fn get_input(name: &SmolStr) -> Option<Arc<str>> {
    let cache = FILE_CACHE.lock().unwrap();
    let res = cache.get(name).cloned();
    drop(cache);
    res.or_else(|| {
        PathBuf::try_from(name.to_string())
            .ok()
            .map(|p| import_file(&p))
    })
}
