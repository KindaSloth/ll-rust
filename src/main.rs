use std::mem;

#[derive(Clone, Debug)]
enum LinkedList<T: Clone + Eq> {
    Nil,
    Cons { value: T, next: Box<LinkedList<T>> }
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
                    next: Box::new(Nil)
                }
            },
            Cons { value: _, next } => { 
                next.append(value)
            },
        }
    }

    fn delete(&mut self, value: T) {
        use LinkedList::*;

        match self {
            Nil => (),
            Cons { value: node_value, next } => {
                if &value == node_value {
                    let x = mem::take(next);

                    *self = *x;

                    return;
                }

                next.delete(value)
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

                next.get(value)
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
            Cons { value: node_value, next } => {
                let value = node_value.clone();

                let x = mem::take(next);

                *self = *x;

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
