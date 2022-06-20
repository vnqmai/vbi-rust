// fn main() {
//     let x = (10, 20);
//     let student_a = Student {
//         name: "Alice".to_string(),
//         age: 18,
//         class: "A1".to_string()
//     };
//     let mut student_b = Student {
//         name: "Bob".to_string(),
//         age: 18,
//         class: "B1".to_string()
//     };

//     student_a.clone().print_age();
//     student_b.clone().print_age();
//     let student_c = Student::new();
//     let name = student_c.get_name();
//     student_c.get_name();
//     Student::print_age(student_a.clone());
//     println!("student_a name: {}", student_a.name);
//     println!("student_b age: {}", student_b.age);
//     student_b.upgrade_age();
//     println!("student_b age: {}", student_b.age);
//     let student_d = Student::new();
//     student_d.get_name();

//     // enum
//     let class_a = Class::A;
//     Class::print_a();
//     let b = Class::get_b();
//     println!("B: {:?}", b);
// }

// #[derive(Clone)]
// struct Student {
//     name: String,
//     age: u8,
//     class: String
// }

// impl Student {
//     fn new() -> Student {
//         Student { name: "Charlie".to_string(), age: 18, class: "C1".to_string() }
//     }

//     // ownership
//     fn print_age(self) {
//         println!("Name: {}", self.name);
//     }

//     // mutable reference
//     fn upgrade_age(&mut self) {
//         self.age += 10;
//     }

//     // shared reference
//     fn get_name(&self) -> String {
//         (*self.name).to_string()
//     }
//     // fn get_name(&self) -> &String {
//     //     &self.name
//     // }
// }


// // Không kiểu dữ liệu
// // Giới hạn
// // Mục đích: chọn
// #[derive(Debug)]
// enum Class {
//     A,
//     B,
//     C
// }

// impl Class {
//     fn print_a() {
//         println!("A: {:?}", Class::A);
//     }
//     fn get_b() -> Class {
//         Self::B
//     }
// }

// enum ClassA {
//     A(u32, u8),
//     B,
//     C
// }

// // lồng struct
// enum ClassC {
//     A(Student),
//     B
// }

// // struct lồng enum
// struct StudentD {
//     name: String,
//     age: u8,
//     class: ClassC
// }

// #[derive(PartialEq)]
// enum Direction { North, East, South, West }

// fn main() {
//     let x = Direction::North;
//     // match x {
//     //     Direction::North => println!("True"),
//     //     Direction::East => println!("False"),
//     //     _ => println!("False")
//     // }

//     if x == Direction::North {
//         println!("True");
//     }
//     else {
//         println!("False");
//     }
// }

// // vec
// fn main() {
//     let mut a: Vec<u8> = Vec::new(); // 1.Sử dụng new() method
//     let mut b: Vec<u8> = vec![]; // 2. Sử dụng vec! macro

//     //Lấy giá trị và thay đổi giá trị
//     let mut c = vec![5, 4, 3, 2, 1];
//     c[0] = 1;
//     c[1] = 2;
//     // c[6] = 8;
//     println!("{}", c[6]);

//     let mut v = vec![1,2,3,4,5];
//     // // .iter(), shared reference
//     // for i in &v {
//     //     println!("A reference to {}", i);
//     // }
//     // for i in v.iter() {
//     //     println!("A reference to {}", i);
//     // }

//     // // for i in v {
//     // //     println!("Take ownership of the vector and its element {}", i);
//     // // }

//     // // iter_mut(), mutable reference
//     // for i in &mut v {
//     //     println!("A mutable reference to {}", i);
//     // }
//     // for i in v.iter_mut() {
//     //     println!("A mutable reference to {}", i);
//     // }

//     // iter(), into_inter(), iter_mut()

//     // IN DISCUSS
//     // // into_inter() -> ownership, mutable reference, shared reference
//     // for i in v.into_iter() {
//     //     println!("i:{}",i);
//     // }
//     // println!("{:?}", v);
// }

// // generic type: kiểu dữ liệu tổng hợp, truyền dữ liệu nào lấy dữ liệu đấy
// fn main() {
//     get_integer_u8(10u8);
//     get_integer(10u32);
//     let student_a = Student::<f64>{
//         name: "A".to_string(),
//         grade: 1.2f64
//     }; 
//     let student_b = Student{
//         name: "A".to_string(),
//         grade: "A".to_string()
//     }; 
//     let student_c = Student::<u8>::new();

//     let x = Some(7);
//     let x_unwrap = x.unwrap();
//     let y: Option<u8> = None;
// }
// // fn get_integer(value: u32) {
// //     println!("{:?}", value);
// // }

// fn get_integer_u32(value: u32) {
//     println!("{:?}", value);
// }

// fn get_integer_u8(value: u8) {
//     println!("{:?}", value);
// }

// fn get_integer<T>(value: T) -> T{
//     // println!("T:{}", value);
//     value
// }

// struct Student<T> {
//     name: String,
//     grade: T
// }

// impl<T> Student<T> {
//     fn new() -> Student<u8> {
//         Student { name: "H".to_string(), grade: 10u8 }
//     }
// }

// struct StudentA<T, V> {
//     name: V,
//     grade: T
// }

// impl<T,V> StudentA<T,V>{}

// enum Class<T> {
//     A(T),
//     B,
//     C
// }

// // Option sự lựa chọn Some(value), None

// // THỰC  HÀNH
// // Bài 1: Điền vào dấu chấm hỏi
// fn main() {
//     let mut shopping_list: Vec<&str> = Vec::new();
//     shopping_list.push("milk");
// }

// // Bài 2: Trường dữ liệu value của Wrapper có thể sử dụng u32 hoặc String hoặc …
// fn main() {

// }
// struct Wrapper<T> {
//     value: T,
// }
// impl<T> Wrapper<T> {
//     pub fn new(value: T) -> Self {
//         Wrapper { value }
//     }
// }


// // Bài 3: 
// // Bước 1: Thực hiện một implement in ra tất cả các trường dữ liệu. 
// // Bước 2: đối với trường dữ liệu grade, ta có thể có trường hợp là “A+”, “B+” ,...
// use std::fmt::Debug;
// fn main() {
//     let reportCard = ReportCard{grade: 8.0, student_age: 8, student_name: "A".to_string()};
//     let reportCard2 = ReportCard{grade: "A+".to_string(), student_age: 8, student_name: "A".to_string()};
//     // reportCard.print_all();
//     reportCard.print_grade();
//     reportCard2.print_grade();
// }


// #[derive(Debug)]
// pub struct ReportCard<T> {
//     pub grade: T,
//     pub student_name: String,
//     pub student_age: u8,
// }

// impl<T: std::fmt::Debug> ReportCard<T> {
//     // fn print_all(self) {
//     //     println!("grade: {}", self.grade);
//     //     println!("student_name: {}", self.student_name);
//     //     println!("student_age: {}", self.student_age);
//     // }
//     fn print_grade(&self) {
//        println!("{:?}", self.grade);
//     }
// }

// // Bài 4: TODO
// #[derive(Debug)]
// enum Message {
//     // TODO: define a few types of messages as used below
//     Quit,
//     Echo,
//     Move,
//     ChangeColor
// }
// fn main() {
//     println!("{:?}", Message::Quit);
//     println!("{:?}", Message::Echo);
//     println!("{:?}", Message::Move);
//     println!("{:?}", Message::ChangeColor);
// }

// // Ví dụ:
// #[derive(Debug)]
// enum Message {
//     // TODO: define the different variants used below
//     Move{x: u32, y:u32},
//     Echo(String),
//     ChangeColor(u32,u32,u32),
//     Quit
// }
// impl Message {
//     fn call(&self) {
//         println!("{:?}", &self);
//     }
// }
// fn main() {
//     let messages = [
//         Message::Move { x: 10, y: 30 },
//         Message::Echo(String::from("hello world")),
//         Message::ChangeColor(200, 255, 255),
//         Message::Quit,
//     ];
//     for message in &messages {
//         message.call();
//     }
// }

// // Ví dụ:
// fn print_number(maybe_number: Option<u16>) {
//     println!("printing: {}", maybe_number.unwrap());
// }
// fn main() {
//     print_number(Some(13));
//     print_number(Some(99));
//     let mut numbers: [Option<u16>; 5] = [None, None, None, None, None];
//     for iter in 0..5 {
//         let number_to_add: u16 = {
//             ((iter * 1235) + 2) / (4 * 16)
//         };
//         numbers[iter as usize] = Some(number_to_add);
//     }
// }

// // ví dụ
// #[derive(Debug, Clone)]
// struct MyData {
//     val1: i32,
//     val2: String,
// }
// impl MyData {
//     pub fn get_val1(&self) -> i32 {
//         return self.val1.clone();
//     }
//     pub fn get_val2(&mut self) -> String {
//         return self.val2.clone();
//     }
//     pub fn get_both(&self) -> (i32, String) {
//         return (self.val1, (*self.val2).to_string());
//     }
// }
// fn main() {
//     let mut d = MyData {
//         val1: 35,
//         val2: String::from("Hello World"),
//     };
//     let both = d.get_both();
//     let x = d.get_val1();
//     let y = d.get_val2();
// }

// Ví dụ
fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}
// &Option<String> -> Option<&String>
struct A {
    p: Option<String>
}
impl A {
    fn a(self) -> Self {
        Self::b(&self.p.as_ref().unwrap());
        self
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}














