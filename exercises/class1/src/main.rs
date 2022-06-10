use std::{io, fs};
use regex::RegexBuilder;
fn main() {
    // 1.  Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
    // Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
    //         let sub_arr = [6,8,10];
    println!("===========BAI TAP 1===========");
    let org_arr = &mut [1, 2,3,5,6,8,10, 11];
    let sub_arr = &mut [6,8,10];
    let result = is_sub_array(org_arr, sub_arr);
    if result == true {
        println!("{:?} is sub array of {:?}", sub_arr, org_arr);
    }
    else {
        println!("{:?} is NOT sub array of {:?}", sub_arr, org_arr);
    }

    // 2.  Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho. 
    // Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
    // https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt
    println!("===========BAI TAP 2===========");
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    let contents = fs::read_to_string("contents.txt").expect("Something went wrong reading the file");
    // let result = contents.matches(&input).count();
    // println!("matches {}", result);

    let re = RegexBuilder::new(&input).case_insensitive(true).build().expect("invalid regex");
    let result = re.find_iter(&contents).count();
    println!("matches {}", result);


}

fn is_sub_array(_org_arr: &mut [i32], _sub_arr: &mut [i32]) -> bool {
    let mut i = 0;
    let mut j = 0;
    let n = _org_arr.len();
    let m = _sub_arr.len();
    while i < n && j < m {
        if _org_arr[i] == _sub_arr[j] {
            i+=1;
            j+=1;

            if j == m {
                return true;
            }
        }
        else {
            i = i - j + 1;
            j = 0;
        }
    }
    return false;
}
