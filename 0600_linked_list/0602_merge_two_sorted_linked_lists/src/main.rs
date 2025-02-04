use rust_linked_list::sixth::LinkedList;
fn main() {
    let list1 = LinkedList::from_iter(vec![1, 2, 4]);
    let list2 = LinkedList::from_iter(vec![1, 3, 4]);

    println!("{:?}", LinkedList::merge_two_lists(list1, list2));
}
