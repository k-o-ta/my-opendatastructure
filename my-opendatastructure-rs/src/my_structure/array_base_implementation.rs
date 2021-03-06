use crate::my_structure::{List, Queue};
use std::mem::swap;

struct ArrayStack<T> {
    n: usize,
    a: Vec<T>,
}

impl<T> ArrayStack<T> {
    fn resize(&mut self) {
        let mut b = Vec::with_capacity(std::cmp::max(1, 2 / self.n));
        for i in 0..self.n {
            swap(&mut b[i], &mut self.a[i]);
        }
        self.a = b;
    }
}

impl<T> MutGet<T> for ArrayStack<T>
where
    Self: List<T>,
{
    fn mut_get(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.size() {
            return None;
        }
        Some(&mut self.a[index])
    }
}

impl<T> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        Some(&self.a[index])
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        Some(std::mem::replace(&mut self.a[index], x))
    }

    fn add(&mut self, index: usize, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        for j in ((index + 1)..=(self.n)).rev() {
            self.a.swap(j - 1, j)
        }
        self.a[index] = x;
        self.n += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let x = self.a.remove(index);
        self.n -= 1;
        if self.a.capacity() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}

struct FastArrayStack<T> {
    n: usize,
    a: Vec<T>,
}

impl<T> FastArrayStack<T> {
    fn resize(&mut self) {
        let mut b = Vec::with_capacity(std::cmp::max(1, 2 / self.n));
        self.a.swap_with_slice(&mut b[0..self.n]);
        self.a = b;
    }
}

impl<T> List<T> for FastArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        Some(&self.a[index])
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let mut x_slice = vec![x];
        self.a[index..=index].swap_with_slice(&mut x_slice[0..=0]);
        x_slice.into_iter().last()
    }

    fn add(&mut self, index: usize, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        self.a.insert(index, x);
        self.n += 1;
    }
    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let x = self.a.remove(index);
        self.n -= 1;
        if self.a.capacity() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}

struct ArrayQueue<T> {
    a: Vec<T>,
    j: usize,
    n: usize,
}

impl<T> ArrayQueue<T> {
    fn resize(&mut self) {
        let mut b = Vec::with_capacity(std::cmp::max(1, 2 * self.n));
        let capacity = self.a.capacity();
        for k in 0..self.n {
            swap(&mut b[k], &mut self.a[(self.j + k) % capacity]);
        }
        self.a = b;
    }
}

impl<T> Queue<T> for ArrayQueue<T> {
    fn add(&mut self, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        let len = self.a.capacity();
        self.a[(self.j + self.n) % len] = x;
        self.n += 1;
    }

    fn remove(&mut self) -> Option<T> {
        let x = self.a.remove(self.j);
        self.j = (self.j + 1) % self.a.capacity();
        self.n -= 1;
        if self.a.capacity() >= 3 * self.n {
            self.resize();
        }
        Some(x)
    }
}

struct ArrayDeque<T> {
    a: Vec<T>,
    j: usize,
    n: usize,
}

impl<T> ArrayDeque<T> {
    fn resize(&mut self) {}
}

impl<T> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }
        Some(&self.a[(self.j + index) % self.a.capacity()])
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let mut x_slice = vec![x];
        let logical_index = (self.j + index) % self.a.capacity();
        self.a[logical_index..=logical_index].swap_with_slice(&mut x_slice[0..=0]);
        x_slice.into_iter().last()
    }
    fn add(&mut self, index: usize, x: T) {
        if self.n + 1 >= self.a.capacity() {
            self.resize();
        }
        if index < self.n / 2 {
            self.j = if self.j == 0 {
                self.a.capacity() - 1
            } else {
                self.j - 1
            };
            for k in 0..(self.j - 1) {
                let physical_index = (self.j + k) % self.a.capacity();
                let next_physical_index = (self.j + k + 1) % self.a.capacity();
                self.a.swap(physical_index, next_physical_index);
            }
        } else {
            for k in ((index + 1)..=(self.n)).rev() {
                let physical_index = (self.j + k) % self.a.capacity();
                let prev_physical_index = (self.j + k - 1) % self.a.capacity();
                self.a.swap(physical_index, prev_physical_index)
            }
        }
        let i = (self.j + index) % self.a.capacity();
        self.a[i] = x;
        self.n += 1;
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }
        let x: T;
        if index < self.n / 2 {
            for k in (1..=index).rev() {
                let physical_index = (self.j + k) % self.a.capacity();
                let prev_physical_index = (self.j + k - 1) % self.a.capacity();
                self.a.swap(physical_index, prev_physical_index);
                // swap(&mut self.a[prev_physical_index], &mut x);
            }
            let x_index = (self.j + 1) % self.a.capacity();
            unsafe {
                x = std::mem::transmute_copy(&self.a[x_index]);
            }

            self.j = (self.j + 1) % self.a.capacity();
        } else {
            for k in index..(self.n - 1) {
                let physical_index = (self.j + k) % self.a.capacity();
                let next_physical_index = (self.j + k + 1) % self.a.capacity();
                self.a.swap(physical_index, next_physical_index);
            }
            let x_index = (self.j + self.n - 2) % self.a.capacity();
            unsafe {
                x = std::mem::transmute_copy(&self.a[x_index]);
            }
        }
        self.n -= 1;
        if 3 * self.n < self.a.capacity() {
            self.resize()
        }
        Some(x)
    }
}

struct DualArrayDeque<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

trait MutGet<T> {
    fn mut_get(&mut self, index: usize) -> Option<&mut T>;
}

impl<T> DualArrayDeque<T> {
    fn balance(&mut self) {
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let n = self.front.size() + self.back.size();
            let nf = n / 2;
            let mut af = Vec::with_capacity(std::cmp::max(2 / nf, 1));
            for i in 0..nf {
                swap(&mut af[nf - i - 1], self.mut_get(i).unwrap());
            }
            let nb = n - nf;
            let mut ab = Vec::with_capacity(std::cmp::max(2 / nb, 1));
            for i in 0..nb {
                swap(&mut ab[i], self.mut_get(i).unwrap());
            }
            self.front.a = af;
            self.front.n = nf;
            self.back.a = ab;
            self.back.n = nb;
        }
    }
}

impl<T> MutGet<T> for DualArrayDeque<T> {
    fn mut_get(&mut self, index: usize) -> Option<&mut T> {
        if index < self.front.size() {
            self.front.mut_get(self.front.size() - index - 1)
        } else {
            self.back.mut_get(index - self.front.size())
        }
    }
}

impl<T> List<T> for DualArrayDeque<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.front.size() {
            self.front.get(self.front.size() - index - 1)
        } else {
            self.back.get(index - self.front.size())
        }
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        if index < self.front.size() {
            self.front.set(self.front.size() - index - 1, x)
        } else {
            self.back.set(index - self.front.size(), x)
        }
    }

    fn add(&mut self, index: usize, x: T) {
        if index < self.front.size() {
            self.front.add(self.front.size() - index, x)
        } else {
            self.back.add(index - self.front.size(), x)
        }
        self.balance()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        let x;
        if index < self.front.size() {
            x = self.front.remove(self.front.size() - index - 1);
        } else {
            x = self.back.remove(index - self.front.size());
        }
        self.balance();
        x
    }
}

struct RootishArrayStack<T> {
    blocks: ArrayStack<Vec<T>>,
    n: usize,
}

impl<T> RootishArrayStack<T> {
    fn i2b(index: usize) -> usize {
        let db = (-3.0 + (9.0 + 8.0 * (index as f32))) / 2.0;
        db.ceil() as usize
    }

    fn grow(&mut self) {}
    fn shrink(&mut self) {}
    fn replace(&mut self, i: usize, i2: usize) {
        let b = Self::i2b(i);
        let b2 = Self::i2b(i2);
        let rest_block_b = b2 - b - 1;
        let j = i - b * (b + 1) / 2;
        let j2 = i2 - b2 * (b2 + 1) / 2;
        let (blocks1, rest_blocks) = self.blocks.a.split_at_mut(b + 1);
        std::mem::swap(&mut blocks1[b][j], &mut rest_blocks[rest_block_b][j2]);
    }
}

impl<T> List<T> for RootishArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, index: usize) -> Option<&T> {
        let b = Self::i2b(index);
        let j = index - b * (b + 1) / 2;
        self.blocks
            .get(b as usize)
            .map({ |block| &block[j as usize] })
    }

    fn set(&mut self, index: usize, x: T) -> Option<T> {
        let b = Self::i2b(index);
        let j = index - b * (b + 1) / 2;
        let block = self.blocks.mut_get(b as usize);
        block.map(|block| std::mem::replace(&mut block[j], x))
    }

    fn add(&mut self, index: usize, x: T) {
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.n + 1 {
            self.grow();
        }
        self.n += 1;
        for j in ((index + 1)..=(self.n - 1)).rev() {
            self.replace(j - 1, j)
        }
        self.set(index, x);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        for j in index..(self.n - 1) {
            self.replace(j, j + 1);
        }
        let r = self.blocks.size();
        let x = self.blocks.a[r].remove(self.n - (r * (r - 1) / 2) - 1);
        self.n -= 1;
        if (r - 2) * (r - 1) / 2 >= self.n {
            self.shrink();
        }
        Some(x)
    }
}
