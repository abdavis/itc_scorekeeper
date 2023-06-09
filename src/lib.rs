mod crdt {
    use core::panic;

    #[derive(Clone)]
    enum ID {
        Base(bool),
        Parent { l: Box<ID>, r: Box<ID> },
    }
    impl ID {
        fn split(&mut self) -> Self {
            match self {
                Self::Base(true) => {
                    *self = Self::Parent {
                        l: Box::new(Self::Base(true)),
                        r: Box::new(Self::Base(false)),
                    };

                    Self::Parent {
                        l: Box::new(Self::Base(false)),
                        r: Box::new(Self::Base(true)),
                    }
                }
                Self::Base(false) => Self::Base(false),
                Self::Parent { l, r } => match (&mut **l, &mut **r) {
                    (Self::Base(false), r) => Self::Parent {
                        l: Box::new(Self::Base(false)),
                        r: Box::new(r.split()),
                    },
                    (l, Self::Base(false)) => Self::Parent {
                        l: Box::new(l.split()),
                        r: Box::new(Self::Base(false)),
                    },
                    (l, r) => {
                        let split = Self::Parent {
                            l: Box::new(Self::Base(false)),
                            r: Box::new(r.clone()),
                        };

                        *self = Self::Parent {
                            l: Box::new(l.clone()),
                            r: Box::new(Self::Base(false)),
                        };
                        split
                    }
                },
            }
        }

        fn join(&mut self, other: Self) {
            match (&mut *self, other) {
                (val @ Self::Base(false), r) => *val = r,
                (_, Self::Base(false)) => (),
                (Self::Parent { l: l1, r: r1 }, Self::Parent { l: l2, r: r2 }) => {
                    l1.join(*l2);
                    r1.join(*r2);
                }
                _ => panic!("tried to merge two overlapping ids together"),
            }
            if let Self::Parent { l, r } = self {
                if let (Self::Base(l), Self::Base(r)) = (&**l, &**r) {
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
