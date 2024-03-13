pub trait RetainIndexed {
    fn retain_indexed<F>(&mut self, f: F)
    where
        F: FnMut(usize, &Self::Item) -> bool;
    type Item;
}

impl<T> RetainIndexed for Vec<T> {
    fn retain_indexed<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &Self::Item) -> bool,
    {
        let mut index = 0;
        self.retain(|item| {
            let result = f(index, item);
            index += 1;
            result
        });
    }

    type Item = T;
}

#[test]
fn test_retain_indexed() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    v.retain_indexed(|index, _| index % 2 == 0);
    assert_eq!(v, vec![1, 3, 5, 7, 9]);
}
