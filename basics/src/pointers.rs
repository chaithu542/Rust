// refernece pointer - point to a resource in the memory

pub fn run(){
    // Primitive array

    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1,arr2));

    //with non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. Youll need to use a reference to point to the resource

    //Vector

    let Vec1 = vec![1,2,3];
    let Vec2 = &Vec1;

    println!("Values: {:?}", (&Vec1,Vec2));
}