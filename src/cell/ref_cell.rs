use crate::cell::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(u32),
    Exclusive,
}
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
                let x = unsafe { &*self.value.get() };

                Some(Ref {
                    value: x,
                    state: &self.state,
                })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                let x = unsafe { &*self.value.get() };

                Some(Ref {
                    value: x,
                    state: &self.state,
                })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                let x = unsafe { &mut *self.value.get() };

                Some(RefMut {
                    value: x,
                    state: &self.state,
                })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                let x = unsafe { &mut *self.value.get() };

                Some(RefMut {
                    value: x,
                    state: &self.state,
                })
            }
            RefState::Exclusive => None,
        }
    }
}

pub struct Ref<'a, T> {
    value: &'a T,
    state: &'a Cell<RefState>,
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        // Update the state
    }
}

pub struct RefMut<'a, T> {
    value: &'a mut T,
    state: &'a Cell<RefState>,
}
