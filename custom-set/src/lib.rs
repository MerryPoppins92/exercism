// use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Clone> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    // phantom: PhantomData<T>,
    v: Vec<T>,
}




impl<T> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self 
    where
        T: Clone
    {
        // unimplemented!();
        let mut cs = CustomSet {
            v: vec![]
        };
        // for i in _input {
        //     let c = &*i;
        //     cs.v.push(*c);
        // }
        cs
    }

    pub fn contains(&self, _element: &T) -> bool {
        unimplemented!();
    }

    pub fn add(&mut self, _element: T) {
        unimplemented!();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!();
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
