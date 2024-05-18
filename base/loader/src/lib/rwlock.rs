/*  rwlock.rs - RwLock implementation
 *
 *  zOS  --  Advanced *NIX System
 *  Copyright (C) 2024  Free Software Foundation, Inc.
 *
 *  zOS is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  zOS is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with zOS. If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

// Define the maximum number of readers
const MAX_READERS: usize = 32;

pub struct RwLock<T: ?Sized> {
    readers: AtomicUsize,
    writers: AtomicUsize,
    data: UnsafeCell<T>,
}
unsafe impl<T> Sync for RwLock<T> {}
//  unsafe impl<T: ?Sized + Send> Send for RwLock<T> {}
//  unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(data: T) -> RwLock<T> {
        RwLock {
            readers: AtomicUsize::new(0),
            writers: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn read(&self) -> Option<RwLockReadGuard<T>> {
        // Increment the reader count
        let mut readers = self.readers.load(Ordering::Relaxed);
        while readers < MAX_READERS && self.writers.load(Ordering::Relaxed) == 0 {
            match self.readers.compare_exchange_weak(
                readers,
                readers + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Some(RwLockReadGuard { lock: self }),
                Err(x) => readers = x,
            }
        }
        None
    }

    pub fn write(&self) -> Option<RwLockWriteGuard<T>> {
        // Try to set the writer flag
        if self.writers.compare_exchange_weak(
            0,
            1,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_ok() {
            // Wait until there are no readers
            while self.readers.load(Ordering::Relaxed) > 0 {}
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }
}

pub struct RwLockReadGuard<'a, T: ?Sized> {
    lock: &'a RwLock<T>,
}

impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

pub struct RwLockWriteGuard<'a, T: ?Sized> {
    lock: &'a RwLock<T>,
}

impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.readers.fetch_sub(1, Ordering::Release);
    }
}

impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.writers.store(0, Ordering::Release);
    }
}


