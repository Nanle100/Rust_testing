// An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

fn main(){


// .iter()
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    /*
    Types of iterators
    1. .iter() : 
    it borrows each element in the collection with out ownership.
    Use Case: when you want to read or inspect elements without taking ownership or modifying the collection.
     
     2. .iter_mut()
     borrows element of a collection and modify them(mutable)

     3. into_iter()
     consumes collections for ownership

     */

     let v1: Vec<i32> = vec![1, 2, 3];

     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
 
     assert_eq!(v2, vec![2, 3, 4]);
 
     
     
     }
