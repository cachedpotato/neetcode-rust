use rust_linked_list::sixth::LinkedList;
fn main() {
    let mut list = LinkedList::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    list.reverse_k_group(3);
    println!("{:?}", list);
}
