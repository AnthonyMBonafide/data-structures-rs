use crate::cell::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(u32),
    Exclusive,
}

/// RefCell allows for runtime borrow checking by dynamically keeping track of the live references.
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(item: T) -> Self {
        RefCell {
            value: UnsafeCell::new(item),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFTEY:
                // This is ok since there are no other references to the underlying data.
                let x = unsafe { &*self.value.get() };

                Some(Ref {
                    value: x,
                    state: &self.state,
                })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                // SAFTEY:
                // This is ok since there are only other shared references and no exclusive
                // references.
                let x = unsafe { &*self.value.get() };

                Some(Ref::new(x, &self.state))
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Exclusive);
                // SAFTEY:
                // This is ok since there are no other references.
                let x = unsafe { &mut *self.value.get() };

                Some(RefMut::new(x, &self.state))
            }
            RefState::Exclusive | RefState::Shared(_) => None,
        }
    }
}

/// Ref encapsulates the reference data
pub struct Ref<'a, T> {
    value: &'a T,
    state: &'a Cell<RefState>,
}

impl<'a, T> Ref<'a, T> {
    fn new(item: &'a T, state: &'a Cell<RefState>) -> Self {
        Ref { value: item, state }
    }
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        // Update the state
        match self.state.get() {
            RefState::Unshared | RefState::Exclusive => unreachable!(),
            RefState::Shared(n) => {
                if n == 1 {
                    self.state.set(RefState::Unshared);
                } else {
                    self.state.set(RefState::Shared(n - 1));
                }
            }
        }
    }
}

/// RefMut encapsulates the mutable references
pub struct RefMut<'a, T> {
    value: &'a mut T,
    state: &'a Cell<RefState>,
}

impl<'a, T> RefMut<'a, T> {
    fn new(item: &'a mut T, state: &'a Cell<RefState>) -> Self {
        RefMut { value: item, state }
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        // Update the state
        match self.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => self.state.set(RefState::Unshared),
        }
    }
}
