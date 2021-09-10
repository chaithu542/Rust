
//enums are types which have a few definite values
pub fn run(){
    let avatar1 = movements::Left;
    let avatar2 = movements::Right;
    let avatar3 = movements::Up;
    let avatar4 = movements::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

enum movements{
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: movements){
    //Perform action depending on info

    match m{
        movements::Up => println!("Avatar Moving Up"),
        movements::Down => println!("Avatar Moving Down"),
        movements::Left => println!("Avatar Moving Left"),
        movements::Right => println!("Avatar Moving Right")
    }
}