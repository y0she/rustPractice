fn main() {
    let x = 20;

    if x > 10 && x < 30 {
        println!("x = {}", x);
        println!("xの値は10より大きい、かつ30より小さいです。");
    }

    if x <= 10 || x >= 30 {
        println!("x = {}", x);
        println!("xの値は10以下、または30以上です。");
    }

    if !(x < 0) {
        println!("x = {}", x);
        println!("xは非負の数です。");
    }
}
