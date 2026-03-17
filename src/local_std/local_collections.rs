use std::collections::{BinaryHeap, VecDeque};

pub fn collection_module_main() -> std::io::Result<()> {
    let vec = vec![23; 4];
    println!("{:?}", vec);

    let mut deque = VecDeque::from([0, 1, -1]);
    deque.push_back(22);
    deque.push_front(0);
    deque.pop_back();
    println!("{:?}", deque);


    let mut binary_h = BinaryHeap::new();
    binary_h.push("Jonh");
    binary_h.push("Jaeoiaeoiaeoia");

    println!("{}", binary_h.peek().unwrap());
    Ok(())
}
