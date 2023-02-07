use std::{
        collections::HashMap,
        sync::{ Arc, Mutex },
        io::{ Error, ErrorKind },
        error::Error as ErrorTrait
};

pub trait StateItem: Sync + Send { }

impl std::fmt::Debug for dyn StateItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("StateItem").finish()
        }
}

pub struct State {
        items: HashMap<String, Arc<Mutex<Box<dyn StateItem>>>>
}

impl State {
        pub fn new() -> Self {
                let items = HashMap::new();
                State { items }
        }

        pub fn add_to_state(&mut self, key: &str, item: impl StateItem + 'static) -> Result<(), Box<dyn ErrorTrait>> {
                if self.items.contains_key(key) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Key already exists")))
                }

                self.items.insert(key.to_string(), Arc::new(Mutex::new(Box::new(item))));

                Ok(())
        }

        pub fn update_state_item(&mut self, key: &str, item: impl StateItem + 'static) -> Result<(), Box<dyn std::error::Error>> {
                if !self.items.contains_key(key) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Key not found")))
                }

                self.items.insert(key.to_string(), Arc::new(Mutex::new(Box::new(item))));

                Ok(())
        }

        pub fn remove_from_state(&mut self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
                match self.items.remove(key) {
                        Some(_) => Ok(()),
                        None => Err(Box::new(Error::new(ErrorKind::Other, "Key not found")))
                }
        }

        pub fn get_item<F: FnOnce(&Box<dyn StateItem>) -> ()>(&self, key: &str, closure: F) -> Result<(), Box<dyn std::error::Error>> {
                let item = match self.items.get(key) {
                        Some(res) => res,
                        None => return Err(Box::new(Error::new(ErrorKind::Other, "Key not found")))
                };

                let item = match Arc::try_unwrap(item.into()) {
                        Ok(res) => res,
                        Err(err) => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to unwrap reference to state item")))
                };

                // If you are hanging in your code by using this get_item method, it is most likely caused by this, which blocks the thread calling it until
                // it CAN get a lock on the MutexGuard<'_, Box<dyn StateItem>> instance. To resolve this, use the try_get_item version of this method, which will
                // return an error if it cannot get a lock on that data at call-time instead of blocking the thread calling it.
                let locked = item.lock().unwrap();

                // Return a reference to the Box<dyn StateItem> on the heap by dereferencing the MutexGuard<'_, Box<dyn StateItem>> object
                // Cannot return the Box<dyn StateItem> dereferenced, as the lock of the Box<_> on the heap must not out live the function that locks it (This function)
                // To deal with this, we give the user a reference to the data on the heap via a closure, which guarantees that this function outlives the closure.
                closure(&*locked);

                Ok(())
        }

        pub fn try_get_item<F: FnOnce(&Box<dyn StateItem>) -> ()>(&self, key: &str, closure: F) -> Result<(), Box<dyn std::error::Error>> {
                let item = match self.items.get(key) {
                        Some(res) => Arc::clone(res),
                        None => return Err(Box::new(Error::new(ErrorKind::Other, "Key not found")))
                };

                let item = match Arc::try_unwrap(item) {
                        Ok(res) => res,
                        Err(_) => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to unwrap reference to state item")))
                };

                let locked = match item.try_lock() {
                        Ok(res) => res,
                        Err(_) => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to get mutable instance of the state item")))
                };

                // Return a reference to the Box<dyn StateItem> on the heap by dereferencing the MutexGuard<'_, Box<dyn StateItem>> object
                // Cannot return the Box<dyn StateItem> dereferenced, as the lock of the Box<_> on the heap must not out live the function that locks it (This function)
                // To deal with this, we give the user a reference to the data on the heap via a closure, which guarantees that this function outlives the closure.
                closure(&*locked);

                Ok(())
        }
}