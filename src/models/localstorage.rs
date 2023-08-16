//! Abstracts the annoying bits of dealing with localStorage

use crate::{
    error::{Error, Result},
};
use web_sys;

/// Get an item out of localStorage
pub fn get(key: &str) -> Option<String> {
    match web_sys::window() {
        Some(window) => {
            match window.local_storage() {
                Ok(Some(storage)) => {
                    match storage.get_item(key) {
                        Ok(item) => item,
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Set a vlaue into localStorage
pub fn set(key: &str, val: &str) -> Result<()> {
    match web_sys::window() {
        Some(window) => {
            match window.local_storage() {
                Ok(Some(storage)) => {
                    storage.set_item(key, val)
                        .map_err(|e| Error::LocalStorage(format!("{:?}", e)))
                }
                Ok(None) => Err(Error::LocalStorage(format!("Missing localStorage"))),
                Err(e) => Err(Error::LocalStorage(format!("{:?}", e))),
            }
        }
        _ => Err(Error::LocalStorage(format!("Missing window"))),
    }
}

/// Remove an item from localStorage
pub fn remove(key: &str) -> Result<()> {
    match web_sys::window() {
        Some(window) => {
            match window.local_storage() {
                Ok(Some(storage)) => {
                    storage.remove_item(key)
                        .map_err(|e| Error::LocalStorage(format!("{:?}", e)))
                }
                Ok(None) => Err(Error::LocalStorage(format!("Missing localStorage"))),
                Err(e) => Err(Error::LocalStorage(format!("{:?}", e))),
            }
        }
        _ => Err(Error::LocalStorage(format!("Missing window"))),
    }
}

