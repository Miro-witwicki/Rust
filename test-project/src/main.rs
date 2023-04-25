use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::ops::Add;
use std::thread;
use std::time::Duration;
mod restaurant;
use crate::restaurant::order_food;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
//data module
mod data {
    #[derive(Debug)]
    pub struct User {
        pub name: String,
        pub age: i32,
        pub email: String,
    }
    impl User {
        pub fn new(name: String, age: i32, email: String) -> User {
            return User { name, age, email };
        }

        pub fn get_name(&self) -> &String {
            return &self.name;
        }

        pub fn get_age(&self) -> i32 {
            return self.age;
        }

        pub fn get_email(&self) -> &String {
            return &self.email;
        }
    }
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    // use crate::data::*;
    // println!("Provide name :");
    // let mut name: String = String::new();
    // io::stdin().read_line(&mut name).expect("Didn`t receive input");
    // let user: User = User {
    //     name,
    //     age: 32,
    //     email: String::from("miro@miro.test")
    // };
    // print!("User details: {}", user.get_name().trim_end());

    //variables
    // let age = "47";
    // let mut age: u32 = age.trim().parse().expect("Age wasn`t assigned a number");
    // age = age + 1;
    // print!("I`m {} years old", age);
    // let _is_available = true;

    //match
    //  let age = 100;
    //  match age {
    //     1..=18 => print!("range"),
    //     21 | 50 => print!("This is another range"),
    //     65..=i32::MAX => print!("Max range number"),
    //     _=> print!("else condition")
    //  }

    //Enums

    //  enum Days {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday
    //  }

    //  impl Days {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Days::Saturday | Days::Sunday => true,
    //             _ => false
    //         }
    //     }
    //  }

    //  let today: Days = Days::Sunday;
    //  match today {
    //     Days::Monday => print!("Everyone hates Monday \n"),
    //     _ => print!("good day \n")
    //  }

    //  print!("Is weekend : {} ", today.is_weekend());

    //vectors

    // let mut vec1 = vec![1,2,3,4,5];
    // vec1.push(8);
    // let second: &u32 = &vec1[1];
    // match vec1.get(1) {
    //     Some(second) => print!("Element : {}", second),
    //     None => print!("Value not found")
    // }

    // for i in &mut vec1 {
    //             *i *= 2;
    // }

    // for i in &vec1 {
    //     print!("{} \n", i)
    // }

    // print!("lenght {}", vec1.len());
    // print!("pop {}", vec1.pop().unwrap());

    //generics

    //  print!("Add two numbers : 10 and 5  = {} \n", get_sum_gen(10, 5) );
    //  print!("Add two numbers : 10.6 and 5.1  = {} \n", get_sum_gen(10.6, 5.1) );

    //hashmap

    // let mut map = HashMap::new();
    // map.insert(1, "Miro");
    // map.insert(2, "Joe");
    // map.insert(3, "Danny");

    // for(k,v) in map.iter() {
    //     if k == &1 {
    //         let member = map.get(k);
    //         match member {
    //             Some(member) => print!("{}", member),
    //             None => print!("Member not found")
    //         }
    //     }
    // }

    //GENERICS

    // trait Shape<T, R> {
    //     fn new(x: T, y: T) -> R;
    //     fn area(&self) -> T;
    // }

    // struct Rectangle {
    //     x: i32,
    //     y: i32
    // }

    // impl Shape<i32, Rectangle> for Rectangle {
    //     fn new(x: i32, y: i32) -> Rectangle {
    //         return Rectangle {
    //             x,
    //             y
    //         }
    //     }

    //     fn area(&self) -> i32 {
    //         return self.x * self.y;
    //     }
    // }

    // struct Square {
    //     x: i32,
    // }

    // impl Shape<i32, Square> for Square {
    //     fn new(x: i32, y: i32) -> Square {
    //         return Square {x}
    //     }

    //     fn area(&self) -> i32 {
    //         return self.x * self.x;
    //     }
    // }

    // let square = Square {
    //     x: 5
    // };
    // let rectangle1 = Rectangle {
    //     x: 2,
    //     y: 3
    // };

    // print!("Rectangle  : x={} y={}  area= {} \n", rectangle1.x, rectangle1.y, rectangle1.area());
    // print!("Square  : x={} area= {}", square.x, square.area());

    //modules
    // order_food();

    //write to file
    // use std::fs::File;
    // use std::io::Write;

    // let path = "C:/Development/projects/rust/sample/lines.txt";
    // let output = File::create(path);
    // let mut result = match output {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem while creating file {:?}", error);
    //     }
    // };
    // write!(result, "Test").expect("Failed to write to the file");

    // //read from the file
    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // for line in buffered.lines() {
    //     print!("{}", line.unwrap());
    // }

    // //write to text sample2
    // let output2 = File::open("C:/Development/projects/rust/sample/data.txt");
    // let mut result2 = match output2 {
    //     Ok(file) => file,
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => File::create("C:/Development/projects/rust/sample/data.txt").unwrap(),
    //             _other => panic!("Cannot open a file {:?}", error)
    //         }
    //     }
    // };
    // write!(result2, "Data");

    //clousures
    // let can_vote = |age:i32| {
    //     return age >= 18;
    // };

    // print!("Can vote {} \n", can_vote(8));

    // let mut samp1 = 5;
    // let print_var = || print!("Sample 1 = {} \n", samp1);
    // print_var();

    // samp1 =10;
    // let mut change_var = || samp1 +=1;
    // change_var();
    // print!("{}", samp1);

    //threads

    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            print!("Current balance does not have enough money requested \n");
        } else {
            bank_ref.balance -= amount;
            print!(
                "Customer widthdrew : {}. Current balance :{} \n",
                amount, bank_ref.balance
            );
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00)
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 100.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        return thread::spawn(|| {
            customer(bank_ref);
        });
    });

    for handle in handles {
        handle.join().unwrap();
    }
    print!("Total {}", bank.lock().unwrap().balance);
}
