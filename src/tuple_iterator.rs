pub(crate) struct PairIter<T> {
    a: Option<T>,
    b: Option<T>,
}

impl<T> PairIter<T> {
    pub(crate) fn new(pair: (T, T)) -> Self {
        Self {
            a: Some(pair.0),
            b: Some(pair.1),
        }
    }
}

impl<T> Iterator for PairIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.a {
            self.a = None;
            Some(a)
        } else if let Some(b) = self.b {
            self.b = None;
            Some(b)
        } else {
            None
        }
    }
}

pub(crate) struct TripletIter<T> {
    a: Option<T>,
    b: Option<T>,
    c: Option<T>,
}

impl<T> TripletIter<T> {
    pub(crate) fn new(triplet: (T, T, T)) -> Self {
        Self {
            a: Some(triplet.0),
            b: Some(triplet.1),
            c: Some(triplet.2)
        }
    }
}

impl<T> Iterator for TripletIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.a {
            self.a = None;
            Some(a)
        } else if let Some(b) = self.b {
            self.b = None;
            Some(b)
        } else if let Some(c) = self.c {
            self.c = None;
            Some(c)
        } else {
            None
        }
    }
}
