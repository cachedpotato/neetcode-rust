use rust_linked_list::sixth::LinkedList;
fn main() {
    let mut list = LinkedList::from_iter(vec![1, 2, 3, 4]);
    list.reverse();
    println!("{:?}", list);

    let another = LinkedList::from_iter(vec![1, 2]);
    println!("{:?}", LinkedList::reverse_list(another));

    let mut empty: LinkedList<i32> = LinkedList::new();
    empty.reverse();
    println!("{:?}", empty);
}
