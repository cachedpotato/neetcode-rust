use rust_linked_list::sixth::LinkedList;
fn main() {
    let list = LinkedList::from_iter(vec![1, 2, 3, 4, 5]);
    println!("{:?}", LinkedList::remove_nth_from_end(list.clone(), 2));
    println!("{:?}", LinkedList::remove_nth_from_end(list.clone(), 5));
    println!("{:?}", LinkedList::remove_nth_from_end(list.clone(), 1));
}
