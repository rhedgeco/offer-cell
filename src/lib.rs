use std::ops::{Deref, DerefMut};

pub struct OfferCell<T> {
    item: Option<T>,
}

impl<T> Default for OfferCell<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> OfferCell<T> {
    pub fn empty() -> Self {
        Self { item: None }
    }

    pub fn new(item: T) -> Self {
        Self { item: Some(item) }
    }

    pub fn item(&self) -> Option<&T> {
        self.item.as_ref()
    }

    pub fn item_mut(&mut self) -> Option<&mut T> {
        self.item.as_mut()
    }

    pub fn offer(&mut self) -> Option<Offered<T>> {
        if self.item.is_none() {
            return None;
        }

        Some(Offered {
            item: &mut self.item,
        })
    }
}

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
    pub fn take(self) -> T {
        match self.item.take() {
            Some(item) => item,
            None => unreachable!(),
        }
    }
}
