use std::ops::{Deref, DerefMut};

/// A data container that can offer ownership of its stored information
pub struct OfferCell<T> {
    item: Option<T>,
}

impl<T> Default for OfferCell<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> OfferCell<T> {
    /// Returns an empty cell
    pub fn empty() -> Self {
        Self { item: None }
    }

    /// Returns a new cell that stores `item`
    pub fn new(item: T) -> Self {
        Self { item: Some(item) }
    }

    /// Returns a reference to the stored item
    pub fn item(&self) -> Option<&T> {
        self.item.as_ref()
    }

    /// Returns a mutable reference to the stored item
    pub fn item_mut(&mut self) -> Option<&mut T> {
        self.item.as_mut()
    }

    /// Offers optional ownership of the stored data as a [`Offered`] item
    pub fn offer(&mut self) -> Option<Offered<T>> {
        // offering expects to always hold data
        // so we must check if there is no data stored first
        if self.item.is_none() {
            return None;
        }

        Some(Offered {
            item: &mut self.item,
        })
    }
}

/// A ownership offering that comes from a [`OfferCell`]
pub struct Offered<'a, T> {
    item: &'a mut Option<T>,
}

impl<'a, T> Deref for Offered<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.item {
            Some(item) => item,
            _ => unreachable!(),
        }
    }
}

impl<'a, T> DerefMut for Offered<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.item {
            Some(item) => item,
            _ => unreachable!(),
        }
    }
}

impl<'a, T> Offered<'a, T> {
    /// Consumes the offering, and takes ownership of the data
    pub fn take(self) -> T {
        match self.item.take() {
            Some(item) => item,
            None => unreachable!(),
        }
    }
}
