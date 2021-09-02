pub fn run(){
    /*let mut C = Color{
        red: 255,
        green: 0,
        blue: 0
    };*/

    let mut C = Colors(255,0,0);

    println!("Color: {} {} {}", C.0,C.1,C.2);

    let mut P = person::new("Mary", "Doe");
    P.set_last_name("Williams");

    //println!("Full Name:{} {}", P.first_name, P.last_name);
    println!("Full Name: {}", P.full_name());
    println!("Full Name: {:?}", P.to_tuple());
}

//traditional struct
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

struct person{
    first_name: String,
    last_name: String
}

impl person{
    fn new(first: &str, last: &str) -> person{
        person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    //set lastName
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String,String){
        (self.first_name, self.last_name)
    }
}

//Tuple Struct

struct Colors(u8,u8,u8);