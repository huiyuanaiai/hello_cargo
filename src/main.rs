// preclude 
// 导入输入输出包
use std::io;
// 导入随机数包
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    // let a :mut = 'dsad'
    // println!("{a}");
    // 生成随机数
    let secred_number = rand::thread_rng().gen_range(1..=50);    
    loop {  
        println!("请猜一个数字");
        let mut guess = String::new();
        // 猜测随机数
        // println!("the secred number : {secred_number}");
        // 随机数代码执行
        let res = io::stdin();
        // println!("res");
        res.read_line(&mut guess)
        // 读取后的值
            .expect("failed to read_line");
        // 这行代码把字符串转为数字并进行匹配
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // 打印出输入的数字
        println!("你的数字是 {guess}");
        match guess.cmp(&secred_number) {
            Ordering::Greater => println!("大了"),
            Ordering::Less => println!("小了"),
            Ordering::Equal => {
                println!("恭喜你，答对了");
                break;
                        }
        }      
    }
}
