use std::ops::{Deref, Index};

type InputUnit = char;

#[derive(Debug)]
pub struct Input {
    pub index: usize,
    data: Box<[InputUnit]>,
}

impl Iterator for Input {
    type Item = InputUnit;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.data.get(self.index).copied() // just a char right
    }
}

impl Deref for Input {
    type Target = [InputUnit];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Index<usize> for Input {
    type Output = InputUnit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Input {
    pub fn new(string: &str) -> Self {
        Input { 
            index: 0,
            data: string.chars().collect::<Box<[InputUnit]>>(),
        }
    }

    pub fn get(&self, index: usize) -> Option<InputUnit> {
        self.data.get(self.index + index).copied()
    }
}