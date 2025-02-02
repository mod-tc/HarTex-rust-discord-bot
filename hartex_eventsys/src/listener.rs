//! # The `listener` Module
//!
//! This module implements listeners for sending events.

use std::sync::Arc;

use dashmap::DashMap;

use futures_channel::mpsc::{
    self,
    UnboundedReceiver,
    UnboundedSender
};

/// # Struct `Listener`
///
/// Represents an event listener.
#[derive(Debug, Clone)]
pub struct Listener<T> {
    pub sender: UnboundedSender<T>
}

/// # Struct `Listeners`
///
/// Represents a series of `Listener`s.
#[derive(Debug, Clone)]
pub struct Listeners<T> {
    inner: Arc<ListenersInner<T>>
}

impl<T> Default for Listeners<T> {
    fn default() -> Self {
        Self {
            inner: Arc::new(ListenersInner::default())
        }
    }
}

impl<T> Listeners<T> {
    /// # Instance Method `Listeners:add`
    ///
    /// Creates a new listener.
    pub fn add(&self) -> UnboundedReceiver<T> {
        let id = self.inner.id + 1;
        let (sender, receiver) = mpsc::unbounded();

        self.inner.listeners.insert(id, Listener {
            sender
        });

        receiver
    }

    /// # Instance Method `Listeners::len`
    ///
    /// Returns the total number of listeners present.
    pub fn len(&self) -> usize {
        self.inner.listeners.len()
    }

    /// # Instance Method `Listeners::listeners`
    ///
    /// Returns all the listeners.
    pub fn listeners(&self) -> &DashMap<u64, Listener<T>> {
        &self.inner.listeners
    }
}

#[derive(Debug)]
struct ListenersInner<T> {
    id: u64,
    listeners: DashMap<u64, Listener<T>>
}

impl<T> Default for ListenersInner<T> {
    fn default() -> Self {
        Self {
            id: 0,
            listeners: DashMap::default()
        }
    }
}
