use std::io;

fn fibonacci(n: u32) -> u32{
    if n == 0 || n == 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    let mut f = 0;
    for _ in 0..n-1 {
        f = a + b;
        a = b;
        b = f;
    }
    f
}

fn main() {
    loop{
        println!("数字を入力してください。");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        let n :u32 = match n.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("フィボナッチ数列の{}番目は{}です。",n,fibonacci(n));
        break;
    }
}

