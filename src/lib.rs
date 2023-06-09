mod crdt {
    use core::panic;

    #[derive(Clone)]
    enum ID {
        Base(bool),
        Parent(Box<(ID, ID)>),
    }
    impl ID {
        fn split(&mut self) -> Self {
            match self {
                Self::Base(true) => {
                    *self = Self::Parent(Box::new((Self::Base(true), Self::Base(false))));
                    Self::Parent(Box::new((Self::Base(false), Self::Base(true))))
                }
                Self::Base(false) => Self::Base(false),
                Self::Parent(val) => match &mut **val {
                    (Self::Base(false), r) => {
                        Self::Parent(Box::new((Self::Base(false), r.split())))
                    }
                    (l, Self::Base(false)) => {
                        Self::Parent(Box::new((l.split(), Self::Base(false))))
                    }
                    (l, r) => {
                        let split = Self::Parent(Box::new((Self::Base(false), r.clone())));

                        *self = Self::Parent(Box::new((l.clone(), Self::Base(false))));

                        split
                    }
                },
            }
        }

        fn join(&mut self, other: Self) {
            match (&mut *self, other) {
                (val @ Self::Base(false), r) => *val = r,
                (_, Self::Base(false)) => (),
                (Self::Parent(children_1), Self::Parent(children_2)) => {
                    children_1.0.join(children_2.0);
                    children_1.1.join(children_2.1);
                }
                _ => panic!("tried to merge two overlapping ids together"),
            }
            if let Self::Parent(children) = self {
                if let (Self::Base(l), Self::Base(r)) = &**children {
                    if l == r {
                        *self = Self::Base(*l);
                    }
                }
            }
        }
    }

    #[derive(Clone, PartialEq)]
    struct Clock {
        val: u32,
        children: Option<Box<(Clock, Clock)>>,
    }

    impl Clock {
        fn normalize(&mut self) {
            if let Some(val) = self.try_normalize() {
                *self = val;
            }
        }
        fn try_normalize(&self) -> Option<Self> {
            todo!()
        }
        fn event(&mut self, id: &ID) {
            fn fill(val: &Clock, id: &ID) -> Option<Clock> {
                todo!()
            }
        }
    }
}
