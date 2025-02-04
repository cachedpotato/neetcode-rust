use rust_linked_list::sixth::LinkedList;
fn main() {
    let lists = vec![LinkedList::from_iter([1, 2, 3]), LinkedList::from_iter([4, 5, 6]), LinkedList::from_iter([7, 8, 9])];
    println!("{:?}", LinkedList::merge_k_lists(lists));
    let lists = vec![
        LinkedList::from_iter([1, 2, 5]), 
        LinkedList::from_iter([4, 5, 9]), 
        LinkedList::from_iter([6, 8, 9])
    ];
    println!("{:?}", LinkedList::merge_k_lists(lists));
}
