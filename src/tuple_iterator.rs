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
            c: Some(triplet.2),
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

pub(crate) struct QuadrupletIter<T> {
    a: Option<T>,
    b: Option<T>,
    c: Option<T>,
    d: Option<T>,
}

impl<T> QuadrupletIter<T> {
    pub(crate) fn new(quadruplet: (T, T, T, T)) -> Self {
        Self {
            a: Some(quadruplet.0),
            b: Some(quadruplet.1),
            c: Some(quadruplet.2),
            d: Some(quadruplet.3),
        }
    }
}

impl<T> Iterator for QuadrupletIter<T>
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
        } else if let Some(d) = self.d {
            self.d = None;
            Some(d)
        } else {
            None
        }
    }
}
