use log::*;
use std::fmt;
use std::marker::PhantomData;

use super::{BadValue, Error, Result};
use super::value::Value;

/// Static identifier for the stack. Used in logging.
pub trait StackId {
    const VALUE: &'static str;
}

pub struct Stack<Id> {
    vec: Vec<Value>,
    max_len: usize,
    _id: PhantomData<Id>,
}

impl<Id: StackId> Stack<Id> {
    pub fn new(max_len: usize) -> Self {
        Self {
            vec: Vec::new(),
            max_len,
            _id: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn top(&self) -> Option<&Value> {
        self.vec.last()
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        trace!("{}: pushing {:?}", Id::VALUE, value);
        if self.len() < self.max_len {
            self.vec.push(value);
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }

    pub fn pop(&mut self) -> Result<Value> {
        if self.is_empty() {
            Err(Error::StackUnderflow)
        } else {
            let last = self.len() - 1;
            let r = self.vec.remove(last);
            trace!("{}: popped {:?}", Id::VALUE, r);
            Ok(r)
        }
    }

    pub fn truncate(&mut self, len: usize) -> Result<()> {
        let old_len = self.vec.len();
        if len <= old_len {
            self.vec.truncate(len);
            trace!("{} stack: truncated from {} to {}", Id::VALUE, old_len, len);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    pub fn get(&self, i: usize) -> Result<&Value> {
        self.vec.get(i).ok_or(Error::BadValue(BadValue::Content))
    }

    pub fn get_mut(&mut self, i: usize) -> Result<&mut Value> {
        self.vec.get_mut(i).ok_or(Error::BadValue(BadValue::Content))
    }
}

impl<Id: StackId> fmt::Debug for Stack<Id> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack(id=\"{}\", max_len={}, values={:?})", Id::VALUE, self.max_len, self.vec)
    }
}