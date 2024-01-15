use std::ops::Index;

pub struct Circular<T, const N: usize> {
    data: [Option<T>; N],
    current: usize,
}

impl<T, const N: usize> Circular<T, N> {
    pub fn new() -> Self {
        Circular {
            data: std::array::from_fn(|_| None),
            current: 0,
        }
    }

    pub fn add(&mut self, v: T) {
        self.current = (self.current + 1) % N;
        self.data[self.current] = Some(v);
    }

    pub fn get(&self, index: usize) -> &Option<T> {
        if index >= N {
            &None
        } else {
            let array_index = (self.current + index) % N;
            &self.data[array_index]
        }
    }

    pub fn last(&self) -> &Option<T> {
        &self.data[self.current]
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        CircularIterator {
            circular: &self,
            current: 0,
        }
    }
}

impl<T, const N: usize> Index<usize> for Circular<T, N> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

pub struct CircularIterator<'a, T, const N: usize> {
    circular: &'a Circular<T, N>,
    current: usize,
}

impl<'a, T, const N: usize> Iterator for CircularIterator<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= N {
            None
        } else {
            self.current += 1;
            if let Some(entry) = self.circular.get(self.current - 1) {
                Some(entry)
            } else {
                self.next()
            }
        }
    }
}
