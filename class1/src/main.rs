use std::io;
use rand::Rng;

fn main() {
    // *** BIẾN *** 
    // // Khai báo biến
    // let x = 5; //immutable
    // let mut y = 10;
    // y = 20;
    // x = 50; // ERROR: cannot assign twice to immutable variable

    // // const - Hằng số --> phải khai báo kiểu dữ liệu và được gán giá trị, tồn tại suốt vòng đời chương trình và ko thay đổi được, snake case
    // // phân biệt hằng số vs immutable variable?
    // let x=10;
    // let x;
    // x = 5;
    // const PI : f32 = 3.14; 

    // // Biến shadowing --> khai báo lại biến mới CÙNG tên nhưng KHÁC kiểu dữ liệu
    // // phân biệt biến shadowing & biến mutable?
    // // Shadowing: tạo ra biến MỚI, CÓ thể thay đổi KIỂU dữ liệu của biến
    // // Mutable: vẫn là biến CŨ, KHÔNG thay đổi KIỂU dữ liệu 
    // let x = 5;
    // {
    //     let x = 10;
    //     println!("{}", x);
    // }
    // let x = "Tom";
    // println!("{}", x);

    // *** KIỂU DỮ LIỆU ***
    // // Scalar: lưu trữ ĐƠN giá trị (integer, float, char, bool,...)
    // let num = 1_000_000;
    // let x = 2; // default: i32
    // let y = 1i16;
    // let z: i8 = 8;
    // println!("{} {} {} {}", num, x, y, z);

    // let fl = 3.6; // default: f64
    // let fl1:f32 = 1.2;
    // let fl2 = 0.5f32;
    // println!("{} {} {}", fl, fl1, fl2);

    // let bl = true;
    // let blf = false;
    // println!("{} {}", bl, blf);

    // Compound: lưu trữ ĐA giá trị (array, tuple, slice)
    // // 1. Array: 
    // // - mảng cố định số lượng phần tử, cùng kiểu dữ liệu
    // // let arr: [i32; 3] = [1,2,3];
    // // - mặc định IMMUTABLE (ko thay đổi giá trị phần tử)
    // // println!("{:?}", arr);
    // // println!("{}", arr[0]);
    // // - destructure aray: let [a,b,c] = arr;
    // let  arr = [1,2,3];
    // println!("{:?}", arr);
    // println!("{}", arr[0]);
    // let [a,b,c] = arr;
    // println!("{} {} {}", a, b, c);

    // // 2. Tupple
    // // - lưu trữ tập hợp giá trị có kích thước cố định, có kiểu dữ liệu khác nhau
    // let tup: (i32, u64, &str) = (-10, 5, "a");
    // println!("{:?}", tup);
    // println!("{}", tup.0);
    // // - destructure tuple: 
    // let (a,b,c) = tup;
    // println!("{} {} {}", a, b, c);

    // // 3. Slice
    // // - Là 1 tham chiếu đến 1 phần hoặc toàn bộ vùng nhớ của đối tượng khác
    // // - Mặc định chỉ có quyền ĐỌC - IMMUATABLE
    // let a = [1,2,3,4];
    // let b = &a;
    // let c = &a[0..4];
    // let d = &a[..];
    // let e = &a[1..3];
    // let f = &a[1..];
    // let g = &a[..3];
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);
    // println!("{:?}", d);
    // println!("{:?}", e);
    // println!("{:?}", f);
    // println!("{:?}", g);

    // // 4. String với &str
    // // - String: lưu trữ 1 chuỗi các ký tự, có kích thước động
    // let name = String::from("Tom");
    // println!("{:?}", name);
    // // - &str: String slice, chỉ có quyền đọc
    // let str1 = "Hello world";
    // let str2 = &str1[0..4];
    // println!("{:?}", str1);
    // println!("{:?}", str2);
     // thằng 2 trỏ thằng 1 -> thay đổi giá trị thằng 1 có thay đổi giá trị thằng 2?
    
    // // // borrowed immutable problem
    // // let mut user_name = String::from("Johny Depp");
    // // let name = &user_name[0..4];
    // // user_name.insert(0, 'A');
    // // println!("{:?}", name);
    // // ------ solutions -------
    // let mut name = String::from("Hello ");
    // let tmp = &mut name;
    // tmp.push_str("AAA");
    // println!("tmp: {}", tmp);
    // // --> giải phóng tmp
    // println!("name: {}", name);
    // // khi 2 thằng trỏ 1 vùng nhớ thì chỉ 1 thằng làm việc đc với vùng nhớ trong 1 lúc

    // ** LUỒNG ĐIỀU KHIỂN (CONTROL FLOW) **
    // // Điều kiện: if else, match
    // let a = 5;
    // if a % 2 == 0 {
    //     println!("{} la so chan", a);
    // }
    // else {
    //     println!("{} la so le", a)
    // }

    // let str = match a {
    //     5 => "la so 5",
    //     _ => "khong pha la so 5"
    // };
    // println!("{:?}", str);

    // Vòng lặp: loop, for, while
    // loop {
    //     if a == 5 {
    //         break;
    //     }
    // }
    // for

    // ** THỰC HÀNH **
    // // 1. Đảo chuỗi
    // let input  = "string";
    // let mut output = String::new();
    // for ch in input.chars() {
    //     output.insert(0, ch);
    // }
    // println!("Reverted string = {} to {}", input, output);
    // println!("Reverted string = {} to {}", input, input.chars().rev().collect::<String>());

    // 2. Viết trò chơi xổ số
    loop {
        let rand_number = rand::thread_rng().gen_range(0..100);
        println!("Result {}", rand_number);

        let mut input  = String::new();
        println!("Please enter a number:");
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        println!("input {}", input);
        let number = input.parse::<i32>().unwrap();
        if number<0 {
            break;
        }
        if number == rand_number {
            println!("You win!");
            break;
        }
        else {
            println!("Try again!");
            break;
        }
    }

    println!("End game!")
}
