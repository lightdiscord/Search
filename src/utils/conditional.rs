pub trait Conditional<T> {
    fn conditional_push(&mut self, element: T, condition: bool);
}

impl<T> Conditional<T> for Vec<T> {
    fn conditional_push(&mut self, element: T, condition: bool) {
        if condition {
            self.push(element);
        }
    }
}
