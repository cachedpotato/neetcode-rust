use rust_linked_list::sixth::LinkedList;
fn main() {
    let l1 = LinkedList::from_iter([1, 2, 3]);
    let l2 = LinkedList::from_iter([4, 5, 6]);

    println!("{:?}", LinkedList::add_two_numbers(l1, l2));

    let l3 = LinkedList::from_iter([1, 2, 3, 4]);
    let l4 = LinkedList::from_iter([5, 6, 7]);
    println!("{:?}", LinkedList::add_two_numbers(l3, l4));
}
