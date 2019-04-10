#[allow(unused_assignments)]
use crate::my_structure::{List, SSet, USet};
#[derive(Debug)]
pub struct MyList<T>(Vec<T>);
#[derive(Debug)]
pub struct MyUSet<T: Eq>(Vec<T>);

impl<T> MyList<T> {
    pub fn new(vec: Vec<T>) -> Self {
        MyList(vec)
    }
}

impl<T> List<T> for MyList<T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn set(&mut self, index: usize, x: T) {
        if let Some(elem) = self.0.get_mut(index) {
            *elem = x;
        }
    }

    fn add(&mut self, index: usize, x: T) {
        self.0.insert(index, x)
    }

    fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }
}

impl<T: Eq> MyUSet<T> {
    pub fn new(vec: Vec<T>) -> Self {
        let mut unique_vec = Vec::with_capacity(vec.len());
        for e in vec {
            if !unique_vec.contains(&e) {
                unique_vec.push(e);
            }
        }
        MyUSet(unique_vec)
    }
}

impl<T: Eq> USet<T> for MyUSet<T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn add(&mut self, x: T) -> bool {
        if self.0.contains(&x) {
            return false;
        }
        self.0.push(x);
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::my_structure::naive_implementation::MyList;
    use crate::my_structure::List;

    #[test]
    fn list_size() {
        let my_list = MyList::new(vec![1, 2, 3]);
        assert_eq!(my_list.size(), 3);
    }

    #[test]
    fn list_get() {
        let my_list = MyList::new(vec![1, 2, 3]);
        assert_eq!(my_list.get(1), Some(&2));
    }

    #[test]
    fn list_set() {
        let mut my_list = MyList::new(vec![1, 2, 3]);
        my_list.set(2, 4);
        assert_eq!(my_list.get(2), Some(&4));
    }

    #[test]
    fn list_add() {
        let mut my_list = MyList::new(vec![1, 2, 3]);
        my_list.add(1, 4);
        assert_eq!(my_list.size(), 4);
        assert_eq!(my_list.get(1), Some(&4));
        assert_eq!(my_list.get(2), Some(&2));
    }

    #[test]
    fn list_remove() {
        let mut my_list = MyList::new(vec![1, 2, 3]);
        assert_eq!(my_list.remove(1), 2);
        assert_eq!(my_list.size(), 2);
        assert_eq!(my_list.get(1), Some(&3));
    }

    use crate::my_structure::naive_implementation::MyUSet;
    use crate::my_structure::USet;
    #[test]
    fn uset_size() {
        let my_uset = MyUSet::new(vec![1, 2, 3]);
        assert_eq!(my_uset.size(), 3);
    }

    #[test]
    fn uset_add() {
        let mut my_uset = MyUSet::new(vec![1, 2, 3]);
        assert_eq!(my_uset.add(4), true);
        assert_eq!(my_uset.add(4), false);
        println!("{:?}", my_uset);
        assert_eq!(my_uset.size(), 4);
    }
}
