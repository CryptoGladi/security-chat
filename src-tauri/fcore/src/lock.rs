//! Module for locking program

use fslock::LockFile;
use log::debug;
use std::path::Path;

#[derive(Debug)]
pub struct Lock {
    /// LockFile for **lifetime** and [drop]
    _file: LockFile,
}

impl Lock {
    /// Locking app
    pub fn new(path: impl AsRef<Path> + Clone) -> Self {
        debug!("new with path: {}", path.as_ref().display());

        let mut file = LockFile::open(path.clone().as_ref()).expect("open file for check lock");
        let successful = file
            .try_lock_with_pid()
            .expect("problem in try_lock_with_pid()");

        assert!(successful,
                "APP IS LOCKED. If is error you can delete {}",
                path.as_ref().display()
            );

        Self { _file: file }
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
        let _lock = Lock::new(temp.child("lockfile"));
    }

    #[test]
    fn lock_after_drop() {
        let temp = TempDir::new().unwrap();
        let path = temp.child("lockfile");

        let first_lock = Lock::new(path.clone());
        drop(first_lock);

        let _second_lock = Lock::new(path.clone());
    }

    #[test]
    fn multi_lock() {
        let temp = TempDir::new().unwrap();

        let _first_lock = Lock::new(temp.child("lockfile1"));
        let _second_lock = Lock::new(temp.child("lockfile2"));
    }

    #[test]
    #[should_panic = "APP IS LOCKED"]
    fn panic_lock() {
        let temp = TempDir::new().unwrap();
        let path = temp.child("lockfile");

        let lock = Lock::new(path.clone());
        let lock_panic = Lock::new(path.clone());

        drop(lock);
        drop(lock_panic);
    }
}
