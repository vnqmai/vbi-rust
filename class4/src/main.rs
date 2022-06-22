// fn main () {
//     {
//         let r;
//         // {
//         //     let x = 5;
//         //     r = &x;
//         // }
//         let x = 5;
//         r = &x;
//         // dangling reference
//         println!("r:{}", r);
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(&string1, string2);
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// // // different lifetime problem
// // fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
// //     if x.len() > y.len() {
// //         x
// //     } else {
// //         y
// //     }
// // }





// Trait giống interface ở ngôn ngữ khác
// fn main() {
//     let x: u32 = 100;
//     let res = x.say_hello();
//     println!("{}", res);

//     let person = Person{};
//     let person2 = Person2{};
//     person.get_hello();
//     person2.get_hello();
//     give_greeting(person);
// }

// // struct Person {}
// // impl Person {
// //     fn say_hello_1(&self) -> String {
// //         "Hello".to_string()
// //     }
// // }
// // struct Person2 {}
// // impl Person2 {
// //     fn say_hello_2(&self) -> String {
// //         "World".to_string()
// //     }
// // }

// trait Speak {
//     fn say_hello(&self) -> String;
//     fn get_hello(&self) {
//         println!("Hello world default");
//     }
// }

// struct Person {}
// impl Speak for Person {
//     fn say_hello(&self) -> String {
//         "Hello".to_string()
//     }
// }
// impl Listen for Person {
//     fn say_listen(&self) -> String {
//         "Hello listen".to_string()
//     }
// }
// struct Person2 {}
// impl Speak for Person2 {
//     fn say_hello(&self) -> String {
//         "World".to_string()
//     }
//     fn get_hello(&self) {
//         println!("Hello person 2");
//     }
// }

// impl Speak for String {
//     fn say_hello(&self) -> String {
//         "String".to_string()
//     }
// }

// impl Speak for u32 {
//     fn say_hello(&self) -> String {
//         "u32".to_string()
//     }
// }

// struct Person4 {}
// fn give_greeting(p: impl Listen) {
//     println!("{}", p.say_listen());
// }
// fn give_greeting_trait_bound<T: Listen + Speak>(p: T) {
//     println!("{}", p.say_listen());
// }
// fn give_greeting_trait_bound_where<T>(p: T) where T: Listen {
//     println!("{}", p.say_listen());
// }

// trait Listen {
//     fn say_listen(&self) -> String;
// }
// // fn give_greeting() {
// //     println!("{}", "Hello World");
// // }
// // trait + function: ko có hành vi rõ ràng

// pub use std::fmt;
// use std::ops::AddAssign;
// fn main() {
//     let class = Class{x: 10};
//     println!("{:?}", class);

//     let mut counter = Counter{x: 0, y: 1};
//     let res1 = counter.next();
//     println!("res1: {}", res1);
//     let res2 = counter.next();
//     println!("res2: {}", res2);
//     let res3 = counter.next();
//     println!("res3: {}", res3);
// }
// struct Class {
//     x: u32
// }
// impl fmt::Debug for Class {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Class").field("x", &self.x).finish()
//     }
// }

// struct Counter<T> {
//     x: T,
//     y: T
// }

// impl<T: Copy+AddAssign> Iterator<T> for Counter<T> {
//     fn next(&mut self) -> T {
//         self.x += self.y;
//         self.x
//     }
// }
// pub trait Iterator<T> {
//     fn next(&mut self) -> T;
// }

// // Thực hành 1
// use std::io;
// fn main() {
//     // let mut input: Vec<&str>;
//     let mut input: Vec<String>;
//     loop {
//         let mut input_text = String::new();
//         println!("Type instruction in the format Add <name> to <department>:");
//         io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
//         let trimmed_text: String = input_text.trim().to_string();
//         // Vec<String> chứ ko phải Vec<&str>
//         // input = trimmed_text.split(" ").collect();
//         input = trimmed_text.split(" ").map(|x|x.to_string()).collect();
//         if input[0] == "Add" && input[2] == "to" {
//             break;
//         } else {
//             println!("Invalid format.");
//         }
//     }
//     println!("{:?}", input);
// }


// // Thực hành 2
// use std::fmt::format;
// trait AppendBar {
//     fn append_bar(self) -> Self;
// }

// impl AppendBar for String {
//     //Add your code here
//     fn append_bar(self) -> Self {
//         // format!("{}Bar", self)
//         self + "Bar" //why self + "Bar".to_string() got error ??
//     }
// }

// fn main() {
//     let s = String::from("Foo");
//     let s = s.append_bar();
//     println!("s: {}", s);
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_foo_bar() {
//         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
//     }

//     #[test]
//     fn is_bar_bar() {
//         assert_eq!(
//             String::from("").append_bar().append_bar(),
//             String::from("BarBar")
//         );
//     }
// }

// Thực hành 3
// CACH 1
// trait AppendBar {
//     fn append_bar(&mut self) -> Self;
// }
// //TODO: Add your code here
// impl AppendBar for Vec<String> {
//     fn append_bar(&mut self) -> Self {
//         self.push("Bar".to_string());
//         self.to_vec()
//     }
// }

// // CACH 2
trait AppendBar {
    fn append_bar(self) -> Self;
}
//TODO: Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut res = self.clone();
        res.push("Bar".to_string());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}

fn main(){

}





