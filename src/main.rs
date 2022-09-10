#[derive(Clone, Debug)]
enum LinkedList<T: Clone + Eq> {
    Nil,
    Cons { value: T, next: Box<LinkedList<T>> }
}

impl<T: Clone + Eq> LinkedList<T> {
    fn new() -> Self {
        LinkedList::Nil
    }

    fn append(&mut self, value: T) -> &Self {
        use LinkedList::*;

        match &self {
            Nil => {
                *self = Cons {
                    value,
                    next: Box::new(Nil)
                };

                self
            },
            Cons { value: node_value, next } => { 
                let mut mut_next = *next.clone();
            
                *self = Cons {
                    value: node_value.clone(),
                    next: Box::new(mut_next.append(value).clone())
                };

                self
            },
        }
    }

    fn delete(&mut self, value: T) -> &Self {
        use LinkedList::*;

        match &self {
            Nil => &Nil,
            Cons { value: node_value, next } => {
                if &value == node_value {
                    *self = *next.clone();

                    return self;
                }

                let mut mut_next = *next.clone();

                *self = Cons {
                    value: node_value.clone(),
                    next: Box::new(mut_next.delete(value).clone())
                };

                self
            }
        }
    }

    fn get(&mut self, value: T) -> Option<T> {
        use LinkedList::*;

        match &self {
            Nil => None,
            Cons { value: node_value, next } => {
                if node_value == &value {
                    return Some(value);
                }

                let mut mut_next = *next.clone();

                mut_next.get(value)
            }
        }
    }
}

impl<T: Clone + Eq> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use LinkedList::*;

        match &self {
            Nil => None,
            Cons { value: node_value, next } => {
                let value = node_value.clone();

                *self = *next.clone();

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

    linked_list.delete("Teste2".into());

    let item = linked_list.get("Teste".into());

    let linked_list_to_vec = linked_list.map(|x| {
        x
    }).collect::<Vec<String>>();

    println!("{:?}", item);
    println!("{:?}", linked_list_to_vec);
}
