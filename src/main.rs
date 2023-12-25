fn main() {
    println!("Hello, World!");
}

struct Pq<T: Ord> {
    values: Vec<T>
}

impl<T: Ord> Pq<T> {
    pub fn new() -> Pq<T> {
        Pq {
            values: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.values.push(value);
        self.values.sort_by(|a, b| b.cmp(a));
    }

    pub fn remove(&mut self) -> Option<T> {
        self.values.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::Pq;

    #[test]
    fn insert_remove() {
        let mut pq = Pq::new();
        pq.insert(10);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_in_order() {
        let mut pq = Pq::new();
        pq.insert(12);
        pq.insert(10);
        assert_eq!(Some(10), pq.remove());
    }

    #[test]
    fn insert_remove_out_of_order() {
        let mut pq = Pq::new();
        pq.insert(10);
        pq.insert(12);
        assert_eq!(Some(10), pq.remove());
    }
}
