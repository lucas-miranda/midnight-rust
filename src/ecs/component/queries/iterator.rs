pub struct ComponentQueryIterator<'a, T : ?Sized> {
    iterator: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> ComponentQueryIterator<'a, T> {
    pub(super) fn new<I: Iterator<Item = T> + 'a>(iterator: I) -> Self {
        Self {
            iterator: Box::new(iterator),
        }
    }
}

impl<'a, T> std::iter::Iterator for ComponentQueryIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}
