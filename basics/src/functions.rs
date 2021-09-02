pub fn run(){
    greeting("hello", "Jane");

    println!("sum: {:?}", add(7, 8));

    //closure

    let add_nums = |n1: i32, n2:i32| n1+n2 ;

    println!("C sums: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str){
    println!("{}{}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32{
    n1+n2
}