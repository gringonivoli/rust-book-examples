pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = 0.0;
        if !self.list.is_empty() {
            self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        let mut collection = AveragedCollection::new();
        collection.add(4);

        assert_eq!(collection.average(), 4.0);
    }

    #[test]
    fn multiple_add() {
        let mut collection = AveragedCollection::new();
        collection.add(4);
        collection.add(5);

        assert_eq!(collection.average(), 4.5);
    }

    #[test]
    fn remove_add() {
        let mut collection = AveragedCollection::new();
        collection.add(4);
        collection.add(5);
        collection.remove();

        assert_eq!(collection.average(), 4.0);
    }

    #[test]
    fn multiple_pops() {
        let mut collection = AveragedCollection::new();
        collection.add(4);
        collection.remove();
        collection.remove();

        assert_eq!(collection.average(), 0.0);
    }
}
