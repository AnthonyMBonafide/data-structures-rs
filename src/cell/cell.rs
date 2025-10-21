use std::cell::UnsafeCell;
/// Cell a structure for interior mutability by only giving out a reference to the underlying data
/// and allowing mutations via references.
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn get(&self) -> T {
        // SAFETY:
        // Since the underyling type is Copy we can safely return a copied version of the
        // underlying data.
        unsafe { *self.value.get() }
    }
}

impl<T> Cell<T> {
    pub fn new(item: T) -> Self {
        Cell {
            value: UnsafeCell::new(item),
        }
    }
    pub fn set(&self, value: T) {
        // SAFETY:
        // There are no references to this data since none are ever given out.
        unsafe { *self.value.get() = value };
    }
}

#[cfg(test)]
mod test {
    use super::Cell;
    #[test]
    fn cell() {
        let c = Cell::new(5);
        assert_eq!(c.get(), 5);
        c.set(6);
        assert_eq!(c.get(), 6);
    }
}
