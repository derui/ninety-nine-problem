use std::io;

fn main() {
    let v = vec![1,2,3,4,5];
    println!("get the last element: {}", last(&v).expect(""));
    let e = vec![];
    println!("get the last element of empty : {}", last(&e).is_none());
}

fn last(v: &Vec<i32>) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    return Some(v[(v.len() - 1)]);
}
