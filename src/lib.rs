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
                    self.val = self.max_val();
                    self.children = None;
                    Ok(self.val)
                }

                (Some(children), ID::Parent(children_ids)) => {
                    let children_fill_results =
                        if let Some(((mixed_child, owned_child), mixed_id)) = match &**children_ids
                        {
                            (ID::Base(true), mixed_id) => {
                                Some(((&mut children.1, &mut children.0), mixed_id))
                            }
                            (mixed_id, ID::Base(true)) => {
                                Some(((&mut children.0, &mut children.1), mixed_id))
                            }
                            _ => None,
                        } {
                            match mixed_child.fill(mixed_id) {
                                Ok(val) => {
                                    owned_child.val = max(owned_child.max_val(), val);
                                    owned_child.children = None;
                                    owned_child.val = max(owned_child.val, val);
                                    Ok(self.val + min(owned_child.val, val))
                                }
                                Err(val) => {
                                    let owned_max = owned_child.max_val();
                                    let new_val = max(owned_max, val);
                                    if new_val > owned_child.val {
                                        owned_child.val = new_val;
                                        owned_child.children = None;
                                        Ok(self.val + min(owned_child.val, val))
                                    } else if owned_max > owned_child.val {
                                        owned_child.val = owned_max;
                                        owned_child.children = None;
                                        Ok(self.val + min(owned_child.val, val))
                                    } else {
                                        Err(self.val + min(owned_child.min_val(), val))
                                    }
                                }
                            }
                        } else {
                            match (
                                children.0.fill(&children_ids.0),
                                children.1.fill(&children_ids.1),
                            ) {
                                (Err(l), Err(r)) => Err(self.val + min(l, r)),
                                (Ok(l), Ok(r)) | (Ok(l), Err(r)) | (Err(l), Ok(r)) => {
                                    Ok(self.val + min(l, r))
                                }
                            }
                        };
                    //todo! normalize the event tree if possible
                    let raise_amount = min(children.0.val, children.1.val);
                    self.val += raise_amount;
                    children.0.val -= raise_amount;
                    children.1.val -= raise_amount;

                    if children.0.val == 0 && children.1.val == 0 {
                        if let (None, None) = (&children.0.children, &children.1.children) {
                            self.children = None;
                        }
                    }

                    children_fill_results
                }
            }
        }

        fn max_val(&self) -> u32 {
            self.val
                + match &self.children {
                    None => 0,
                    Some(children) => max(children.0.max_val(), children.1.max_val()),
                }
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
