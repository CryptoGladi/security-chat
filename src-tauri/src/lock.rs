//! Locking program

use fslock::LockFile;
use log::{debug, warn};
use std::{
    fs::{remove_file, File},
    path::Path,
};

#[derive(Debug)]
pub struct Lock {
    file: LockFile,
}

impl Lock {
    pub fn new(path: impl AsRef<Path> + Clone) -> Self {
        debug!("new with path: {}", path.as_ref().display());

        if path.as_ref().is_file() {
            remove_file(path.clone()).expect("APP IS LOCKED!");
        }

        //let file = File::create(path).expect("open file for check lock");
        Self {
            file: LockFile::open(path.clone().as_ref()).expect("open file for check lock"),
        }
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        debug!("unlocking file");
        self.file.unlock().expect("error in unlocking");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;
    use test_log::test;

    #[test]
    fn lock() {
        let temp = TempDir::new().unwrap();
        let lock = Lock::new(temp.child("lockfile"));

        drop(lock);
    }

    #[test]
    #[should_panic]
    fn panic_lock() {
        let temp = TempDir::new().unwrap();
        let lock = Lock::new(temp.child("lockfile"));
        let lock_panic = Lock::new(temp.child("lockfile"));
    }
}
