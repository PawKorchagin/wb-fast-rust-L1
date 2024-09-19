use std::collections::HashSet;

fn main() {
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set2: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    let intersection: HashSet<i32> = set1.intersection(&set2).cloned().collect();

    println!("Intersection of sets set1 and set2 is: {:?}", intersection);
}
