fn main() {
    let x = two_sum([3,2,4].to_vec(), 6);
    println!("{:?}", x);

    let V: Vec<i32> = [3,2,4].to_vec();
    let length: i32 = V.len() as i32;

    for X in V.iter(){
        println!("{}", X);
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    let mut vec : Vec<i32> = Vec::new();

   for x in &nums{
        let mut diff = target - x;
        if diff > 1{
            for y in &nums{
                if x == y {
                } else if x+y == target{
                    vec.push(*x);
                }
            }
        }   
    }
    println!("{:?}", vec.to_vec());
    vec
}

    

    //println!("final list: {:?}", vec);
