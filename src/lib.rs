use std::{
    cell::{Ref as StdRef, RefCell, RefMut as StdRefMut},
    ops::{Deref, DerefMut},
    rc::Rc,
};

/// Remembers the borrower and removes it when Ref / RefMut is dropped.
#[derive(Debug)]
pub struct Dropper {
    borrower: String,
    borrowers: Rc<RefCell<Vec<String>>>,
}

impl Drop for Dropper {
    fn drop(&mut self) {
        let index = self
            .borrowers
            .borrow()
            .iter()
            .position(|b| *b == self.borrower)
            .expect(&format!(
                "Cannot find a borrower with name: {}",
                self.borrower
            ));
        self.borrowers.borrow_mut().remove(index);
    }
}

/// Wraps Ref with Dropper to remove the borrower from the borrowers list automatically.
#[derive(Debug)]
pub struct Ref<'a, T> {
    value: StdRef<'a, T>,
    _dropper: Dropper,
}

impl<'a, T> Ref<'a, T> {
    pub fn new(value: StdRef<'a, T>, borrower: &str, borrowers: Rc<RefCell<Vec<String>>>) -> Self {
        Self {
            value,
            _dropper: Dropper {
                borrower: borrower.to_string(),
                borrowers,
            },
        }
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

/// Wraps RefMut with Dropper to remove the borrower from the borrowers list automatically.
#[derive(Debug)]
pub struct RefMut<'a, T> {
    value: StdRefMut<'a, T>,
    _dropper: Dropper,
}

impl<'a, T> RefMut<'a, T> {
    pub fn new(
        value: StdRefMut<'a, T>,
        borrower: &str,
        borrowers: Rc<RefCell<Vec<String>>>,
    ) -> Self {
        Self {
            value,
            _dropper: Dropper {
                borrower: borrower.to_string(),
                borrowers,
            },
        }
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// Allow users to borrow / borrow_mut the value while keeping the name of the borrower until it's dropped.
/// If the value is borrowed twice, it will return an Err containing the name of the previous borrower.
#[derive(Debug)]
pub struct Lender<T> {
    value: Rc<RefCell<T>>,
    borrowers: Rc<RefCell<Vec<String>>>,
}

impl<T> Lender<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            borrowers: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn borrow(&self, borrower: &str) -> Result<Ref<T>, String> {
        if let Ok(value) = self.value.try_borrow() {
            self.borrowers.borrow_mut().push(borrower.to_string());
            Ok(Ref::new(value, borrower, self.borrowers.clone()))
        } else {
            Err(format!(
                "Failed to borrow immutable reference. Currently borrowed by: {:?}",
                self.borrowers.borrow()
            ))
        }
    }

    pub fn borrow_mut(&self, borrower: &str) -> Result<RefMut<T>, String> {
        if let Ok(value) = self.value.try_borrow_mut() {
            self.borrowers.borrow_mut().push(borrower.to_string());
            Ok(RefMut::new(value, borrower, self.borrowers.clone()))
        } else {
            Err(format!(
                "Failed to borrow mutable reference. Currently borrowed by: {:?}",
                self.borrowers.borrow()
            ))
        }
    }
}

// We can't derive(Clone) because we don't want T to be cloned.
impl<T> Clone for Lender<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            borrowers: self.borrowers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lender;

    #[test]
    fn borrow_twice() -> Result<(), String> {
        let d = Lender::new(1);

        let b1 = d.borrow("b1")?;
        let b2 = d.borrow("b2")?;

        assert_eq!(*b1, 1);
        assert_eq!(*b2, 1);

        Ok(())
    }

    #[test]
    fn borrow_once_borrow_mut_once() -> Result<(), String> {
        let d = Lender::new(1);

        let b1 = d.borrow("b1")?;
        let b2 = d.borrow_mut("b2");

        assert!(b2.is_err());
        if let Err(b) = b2 {
            b.contains("Currently borrowed by: [\"b1\"]");
        }

        Ok(())
    }

    #[test]
    fn borrow_twice_borrow_mut_once() -> Result<(), String> {
        let d = Lender::new(1);

        let b1 = d.borrow("b1")?;
        let b2 = d.borrow("b2")?;
        let b3 = d.borrow_mut("b3");

        assert!(b3.is_err());
        if let Err(b) = b3 {
            b.contains("Currently borrowed by: [\"b1\", \"b2\"]");
        }

        Ok(())
    }

    #[test]
    fn borrow_mut_once_borrow_once() -> Result<(), String> {
        let d = Lender::new(1);

        let b1 = d.borrow_mut("b1")?;
        let b2 = d.borrow("b2");

        assert!(b2.is_err());
        if let Err(b) = b2 {
            b.contains("Currently borrowed by: [\"b1\"]");
        }

        Ok(())
    }

    #[test]
    fn borrow_mut_twice() -> Result<(), String> {
        let d = Lender::new(1);

        let b1 = d.borrow_mut("b1")?;
        let b2 = d.borrow_mut("b2");

        assert!(b2.is_err());
        if let Err(b) = b2 {
            b.contains("Currently borrowed by: [\"b1\"]");
        }

        Ok(())
    }

    #[test]
    fn clone_and_borrow_mut() -> Result<(), String> {
        let d = Lender::new(1);
        let b1 = d.borrow_mut("b1")?;

        let d2 = d.clone();
        let b2 = d.borrow_mut("b2");

        assert!(b2.is_err());
        if let Err(b) = b2 {
            b.contains("Currently borrowed by: [\"b1\"]");
        }

        Ok(())
    }
}
