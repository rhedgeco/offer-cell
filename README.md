# Offer Cell

A rust library that defines a pattern for providing a reference to stored data, and optionally transferring ownership of that data.

## Usage

### Initialization

```rust
// a cell may be created
let cell = OfferCell::new(42);

// or initialzed as empty
let empty = OfferCell::empty();
```

### Accessing Data

```rust
// access the item as a reference
match cell.item() {
    Some(value) = (), // do something with the value
    None => (), // returns none if there is no item
}

// access the item as a mutable reference
match cell.item_mut() {
    Some(value) = (), // do something with the value
    None => (), // returns none if there is no item
}
```

### Offering Data

What sets this apart, is the data within the cell can be _"offered"_

```rust
// if the cell contains an item, it can be offered
let offered = cell.offer() {
    Some(offered) => offered,
    None => return,
};

// the offered item implements Deref and DerefMut
assert_eq!(offered.deref(), &42);

// if nothing else is done with the offered item,
// the data will stay in the cell for later

// alternatively the offering can be consumed
// this leaves nothing in the cell, and takes ownership of the data
let data = offered.take();
```
