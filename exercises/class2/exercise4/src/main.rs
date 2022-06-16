
// //Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)

fn main(){
    let mut a = vec![1,2,3,4,5];
    // let i = 0;
    let c = 0;
    let (a, c) = test(&mut a);
    println!("{}",c);
    println!("{:?}", a);
    // loop {
    //     let (a, c) = test(&mut a);
    //     println!("{}",c);
    //     println!("{:?}", a);
    //     if i >=5 {break;}
    // }
}

pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}
// a=[1,2,3,4,5]    b=[]    c=0
// a.len()=5    a=[1,2,3,4]    d=5 c=5  b=[5]
// a.len()=4    a=[1,2,3]      d=4 c=9  b=[5,4]
// a.len()=3    a=[1,2]        d=3 c=12 b=[5,4,3]
// a.len()=2    a=[1]          d=2 c=14 b=[5,4,3,2]
// a.len()=1    a=[]           d=1 c=15 b=[5,4,3,2,1]
// a.len()=0  --> ([5,4,3,2,1], 15)       