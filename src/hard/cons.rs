#[derive(Debug, PartialEq, Eq)]
enum Cons<T: Clone> {
    Cons(T, Box<Cons<T>>),
    Null
}

impl<T: Clone> Cons<T> {
    pub fn new(head: T, tail: Self) -> Self {
        Cons::Cons(head, Box::new(tail))
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            &Cons::Null => vec![],
            &Cons::Cons(ref head, ref tail) => {
                let mut head = vec![head.clone()];
                head.extend(tail.to_vec());
                head
            }
        }
    }
}

impl<T: Clone> Cons<T> {
    pub fn from_iter<I>(it: I) -> Self
        where I: IntoIterator<Item=T>
    {
        let mut it = it.into_iter();
        match it.next() {
            None => {
                Cons::Null
            }
            Some(value) => {
                Self::Cons(value, Box::new(Self::from_iter(it)))
            }
        }
    }

    pub fn filter<F>(&self, fun: F) -> Self
        where F: Fn(&T) -> bool
    {
        match self {
            Cons::Cons(value, tail) => {
                if !fun(value) {
                    tail.filter(fun)
                } else {
                    Cons::Cons(value.clone(), Box::new(tail.filter(fun)))
                }
            }
            Cons::Null => {
                Cons::Null
            }
        }
    }

    pub fn map<F,S>(&self, fun: F) -> Cons<S>
        where F: Fn(T) -> S, S: Clone
    {
        match self {
            Cons::Cons(value, tail) => {
                Cons::Cons(fun(value.clone()), Box::new(tail.map(fun)))
            }
            Cons::Null => { Cons::Null }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_from_vec() {
        assert_eq!(Cons::from_iter(Vec::<i32>::new()), Cons::Null);

        assert_eq!(Cons::from_iter(vec![1,2,3,4,5]).to_vec(),
                   vec![1,2,3,4,5]);
    }


    #[test]
    fn should_filter() {
        assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
                       .filter(|&n| n > 3)
                       .to_vec(),
                   vec![4,5]);

        assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
                       .filter(|&n| n > 5),
                   Cons::Null);
    }


    #[test]
    fn should_map() {
        assert_eq!(Cons::from_iter(vec!["1","2","3","4","5"])
                       .map(str::parse::<i32>)
                       .map(Result::unwrap)
                       .to_vec(),
                   vec![1,2,3,4,5]);
    }


    #[test]
    fn should_filter_map() {
        assert_eq!(Cons::from_iter(vec![1,2,3,4,5])
                       .filter(|n| n % 2 == 0)
                       .map(|x| x.to_string())
                       .to_vec(),
                   ["2","4"]);
    }
}