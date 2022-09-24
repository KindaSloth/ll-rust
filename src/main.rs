use std::{rc::Rc, cell::RefCell};

#[derive(Clone, Debug)]
enum LinkedList<T: Clone + Eq> {
    Nil,
    Cons { value: T, next: Rc<RefCell<LinkedList<T>>> }
}

impl<T: Clone + Eq> Default for LinkedList<T> {
    fn default() -> Self {
        use LinkedList::Nil;

        Nil
    }
}

impl<T: Clone + Eq> LinkedList<T> {
    fn new() -> Self {
        LinkedList::Nil
    }

    fn append(&mut self, value: T) {
        use LinkedList::*;

        match self {
            Nil => {
                *self = Cons {
                    value,
                    next: Rc::new(RefCell::new(Nil))
                }
            },
            Cons { value: _, next } => { 
                next.borrow_mut().append(value)
            },
        }
    }

    fn get_value(&self) -> Option<T> {
        use LinkedList::*;

        match self {
            Nil => None,
            Cons { value, next: _ } => Some(value.clone())
        }
    }

    fn get_next(&self) -> Option< Rc<RefCell<LinkedList<T>>>> {
        use LinkedList::*;

        match self {
            Nil => None,
            Cons { value: _, next } => Some(Rc::clone(next))
        }
    }

    fn delete(&mut self, value: T) {
        use LinkedList::*;

        match self {
            Nil => (),
            Cons { value: node_value, next: ref_next } => {
                if &value == node_value {
                    let next = Rc::clone(ref_next);

                    let borrowed_next = next.borrow();

                    if let (Some(x), Some(y)) = (borrowed_next.get_value(), borrowed_next.get_next()) {
                        *self = Cons { value: x, next: y }
                    } else {
                        *self = Nil
                    }

                    return;
                }

                ref_next.borrow_mut().delete(value)
            }
        }
    }

    fn get(&mut self, value: T) -> Option<T> {
        use LinkedList::*;

        match self {
            Nil => None,
            Cons { value: node_value, next } => {
                if node_value == &value {
                    return Some(value);
                }

                next.borrow_mut().get(value)
            }
        }
    }
}

impl<T: Clone + Eq> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use LinkedList::*;

        match self {
            Nil => None,
            Cons { value: node_value, next: ref_next } => {
                let value = node_value.clone();

                let next = Rc::clone(ref_next);

                let borrowed_next = next.borrow();

                if let (Some(x), Some(y)) = (borrowed_next.get_value(), borrowed_next.get_next()) {
                    *self = Cons { value: x, next: y }
                } else {
                    *self = Nil
                }

                Some(value)
            }
        }
    }
}

fn main() {
    let mut linked_list: LinkedList<String> = LinkedList::new();

    linked_list.append("Teste".into());
    linked_list.append("Teste2".into());
    linked_list.append("Teste3".into());

    linked_list.delete("Teste3".into());

    let item = linked_list.get("Teste".into());

    println!("{:?}", linked_list);
    println!("{:?}", item);

    let linked_list_to_vec = linked_list.map(|x| {
        x
    }).collect::<Vec<String>>();

    println!("{:?}", linked_list_to_vec);
}
