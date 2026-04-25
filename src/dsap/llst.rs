#[derive(Debug)]
struct Node<T> {
    data: T,
    next_node: Option<usize>,
}

pub struct Iter<'a, T> {
    list: &'a SinglyLinkedList<T>,
    next_idx: Option<usize>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Slot<T> {
    Occupied(Node<T>),
    Vacant { next_free: Option<usize> },
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    nodes: Vec<Slot<T>>,
    head_idx: Option<usize>,
    tail_idx: Option<usize>,
    free_head: Option<usize>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head_idx: None,
            tail_idx: None,
            free_head: None,
            len: 0,
        }
    }

    fn node(&self, idx: usize) -> &Node<T> {
        match &self.nodes[idx] {
            Slot::Occupied(n) => n,
            Slot::Vacant { .. } => {
                panic!("invariant violation: idx {} points to vacant slot", idx)
            }
        }
    }

    fn node_mut(&mut self, idx: usize) -> &mut Node<T> {
        match &mut self.nodes[idx] {
            Slot::Occupied(n) => n,
            Slot::Vacant { .. } => {
                panic!("invariant violation: idx {} points to vacant slot", idx)
            }
        }
    }

    fn allocate(&mut self, node: Node<T>) -> usize {
        match self.free_head {
            Some(idx) => {
                let next_free = match &self.nodes[idx] {
                    Slot::Vacant { next_free } => *next_free,
                    Slot::Occupied(_) => {
                        panic!("free_head pointed to an occupied slot")
                    }
                };
                self.free_head = next_free;
                self.nodes[idx] = Slot::Occupied(node);
                idx
            }
            None => {
                let idx = self.nodes.len();
                self.nodes.push(Slot::Occupied(node));
                idx
            }
        }
    }

    fn free(&mut self, idx: usize) -> Node<T> {
        let old = std::mem::replace(
            &mut self.nodes[idx],
            Slot::Vacant {
                next_free: self.free_head,
            },
        );
        self.free_head = Some(idx);
        match old {
            Slot::Occupied(node) => node,
            Slot::Vacant { .. } => panic!("freeing already-vacant slot at idx {}", idx),
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_node = Node {
            data,
            next_node: None,
        };
        let new_idx = self.allocate(new_node);
        match self.tail_idx {
            Some(old_tail) => self.node_mut(old_tail).next_node = Some(new_idx),
            None => self.head_idx = Some(new_idx),
        }
        self.tail_idx = Some(new_idx);
        self.len += 1;
    }

    pub fn push_front(&mut self, data: T) {
        let curr_head = self.head_idx;
        let new_node = Node {
            data,
            next_node: curr_head,
        };
        let new_idx = self.allocate(new_node);
        if curr_head.is_none() {
            self.tail_idx = Some(new_idx);
        }
        self.head_idx = Some(new_idx);
        self.len += 1;
    }

    pub fn iterate(&self) -> Vec<&T> {
        let mut v = Vec::with_capacity(self.len);
        let mut current = self.head_idx;
        while let Some(idx) = current {
            let node = self.node(idx);
            v.push(&node.data);
            current = node.next_node;
        }
        v
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head_idx.map(|i| &self.node(i).data)
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail_idx.map(|i| &self.node(i).data)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let head_idx = self.head_idx?;

        let next = self.node(head_idx).next_node;
        self.head_idx = next;
        if next.is_none() {
            self.tail_idx = None;
        }
        let n = self.free(head_idx);
        self.len -= 1;
        Some(n.data)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let tail_idx = self.tail_idx?;

        if self.head_idx == self.tail_idx {
            self.head_idx = None;
            self.tail_idx = None;
            let node = self.free(tail_idx);
            self.len -= 1;
            return Some(node.data);
        }

        let mut curr = self.head_idx?;
        loop {
            let next = self.node(curr).next_node;
            if next == Some(tail_idx) {
                break;
            }
            curr = next.expect("walked off list without finding tail");
        }

        self.node_mut(curr).next_node = None;
        self.tail_idx = Some(curr);
        let node = self.free(tail_idx);
        self.len -= 1;
        Some(node.data)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            next_idx: self.head_idx,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.next_idx?;
        let node = self.list.node(idx);
        self.next_idx = node.next_node;
        Some(&node.data)
    }
}

impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

pub fn llist() {
    let mut ll = SinglyLinkedList::<usize>::new();

    assert_eq!(ll.pop_front(), None);
    assert_eq!(ll.pop_back(), None);

    ll.push_back(1);
    assert_eq!(ll.pop_front(), Some(1));
    assert_eq!(ll.len(), 0);
    assert_eq!(ll.peek_front(), None);

    ll.push_back(1);
    assert_eq!(ll.pop_back(), Some(1));
    assert_eq!(ll.len(), 0);

    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);
    assert_eq!(ll.pop_front(), Some(1));
    assert_eq!(ll.pop_back(), Some(3));
    assert_eq!(ll.iterate(), vec![&2]);
    assert_eq!(ll.len(), 1);

    let mut ll = SinglyLinkedList::<usize>::new();
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);
    ll.pop_front();
    ll.push_back(4);
    println!("{:?}", ll);
    assert_eq!(ll.iterate(), vec![&2, &3, &4]);

    for i in 0..100 {
        ll.push_back(i);
    }
    for _ in 0..50 {
        ll.pop_front();
    }
    assert_eq!(ll.len(), ll.iterate().len());

    for x in &ll {
        println!("{}", x);
    }

    let sum: usize = ll.iter().sum();
    let doubled: Vec<usize> = ll.iter().map(|&x| x * 2).collect();
    let count = ll.iter().count();

    println!("sum: {}, doubled: {:?}, count: {}", sum, doubled, count);
    // assert_eq!(1, 2);
}
