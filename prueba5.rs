fn main() {
    let nums = vec![10, 20, 30, 40];

    for n in nums {
        match n {
            30 => println!("thirty"),
            _ => println!("{:?}", n),
        }
    }

    println!("Number of elements: {:?}", nums.len());
}