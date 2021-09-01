// used to check the condition of something

pub fn run(){
    let age = 18;
    let check_id:bool = false;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Allowed to Drink");
    } else if age < 21 && check_id{
        println!("Sorry, you have to leave");
    } else {
        println!("I need to see your id");
    }
}