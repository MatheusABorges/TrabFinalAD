use std::collections::VecDeque;
fn main(){
    let mut q = VecDeque::new();
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    q.pop_front();
    q.pop_front();
    q.push_back(4);
    q.push_back(5);
    q.push_back(6);
    q.push_back(7);

    let max = q.len();
    for i in 0..max{
        println!("{:?}", q.pop_front().unwrap());
    }
}