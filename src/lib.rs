use std::ops::{Deref, DerefMut};

/// A data container that can offer optional ownership of its stored information
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

    /// Returns true if the cell is empty
    pub fn is_empty(&self) -> bool {
        self.item.is_none()
    }

    /// Returns a reference to the stored item
    pub fn item(&self) -> Option<&T> {
        self.item.as_ref()
    }

    /// Returns a mutable reference to the stored item
    pub fn item_mut(&mut self) -> Option<&mut T> {
        self.item.as_mut()
    }

    /// Returns the stored data leaving nothing in its place
    pub fn take(&mut self) -> Option<T> {
        self.item.take()
    }

    /// Inserts `item` into the cell and returns the item as an offering
    pub fn insert_offer(&mut self, item: T) -> Offering<T> {
        self.item = Some(item);
        Offering {
            item: &mut self.item,
        }
    }

    /// Offers optional ownership of the stored data as a [`Offering`] item
    pub fn offer(&mut self) -> Option<Offering<T>> {
        // offering expects to always hold data
        // so we must check if there is no data stored first
        if self.item.is_none() {
            return None;
        }

        Some(Offering {
            item: &mut self.item,
        })
    }
}

/// A ownership offering that comes from a [`OfferCell`]
pub struct Offering<'a, T> {
    item: &'a mut Option<T>,
}

impl<'a, T> Deref for Offering<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.item {
            Some(item) => item,
            _ => unreachable!(),
        }
    }
}

impl<'a, T> DerefMut for Offering<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.item {
            Some(item) => item,
            _ => unreachable!(),
        }
    }
}

impl<'a, T> Offering<'a, T> {
    /// Consumes the offering, and takes ownership of the data
    pub fn take(self) -> T {
        match self.item.take() {
            Some(item) => item,
            None => unreachable!(),
        }
    }
}
