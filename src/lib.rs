mod crdt {
    use std::cmp::{max, min};

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
        pub fn event(&mut self, id: &ID) {}

        fn fill(&mut self, id: &ID) -> Result<u32, u32> {
            match (&mut self.children, id) {
                (_, ID::Base(false)) => Err(self.min_val()),
                (None, _) => Err(self.val),

                (Some(children), ID::Base(true)) => {
                    self.val += Self::max_val(&children.0, &children.1);
                    self.children = None;
                    Ok(self.val)
                }

                (Some(children), ID::Parent(children_ids)) => match &**children_ids {
                    (ID::Base(true), id_r) => todo!(),
                    (id_l, ID::Base(true)) => todo!(),
                    (id_l, id_r) => todo!(),
                },
            }
        }

        fn max_val(l: &Self, r: &Self) -> u32 {
            max(
                l.val
                    + match &l.children {
                        None => 0,
                        Some(children) => Self::max_val(&children.0, &children.1),
                    },
                r.val
                    + match &r.children {
                        None => 0,
                        Some(children) => Self::max_val(&children.0, &children.1),
                    },
            )
        }
        fn min_val(&self) -> u32 {
            self.val
                + match &self.children {
                    None => 0,
                    Some(children) => min(children.0.min_val(), children.1.min_val()),
                }
        }
    }
}
