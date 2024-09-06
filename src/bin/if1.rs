fn main() {
    let x = 20;

    if x > 10 {
        println!("x = {}", x);
        println!("xの値は10より大きいです。");
    }
    if (x + 30) >= 35 {
        println!("x = {}", x);
        println!("x+30の値は35以上です。");
    }

    if true {
        println!("条件が真なので必ず実行されます。");
    }
}
