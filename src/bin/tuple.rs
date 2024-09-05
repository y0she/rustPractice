fn main(){
    //平面上の点(x,y)
    let p = (10, 25);
    println!("(x, y) = ({}, {})",p.0,p.1);

    //3次元空間上の点(x,y,z)
    let q = (5, 10, 30);
    println!("(x, y, z) = ({}, {}, {})",q.0, q.1, q.2);

    //異なる科目の評価と合否(Math, English, Verbal, Result)
    let s: (i32, i32, i32, bool) = (80, 90, 85, true);

    //それぞれの要素を取り出して変数に格納する
    let (math, english, verbal, result) = s;
    println!("(数学、英語、国語、合否) = ({}, {}, {}, {})", math, english, verbal, result);

    //必要な要素だけ取り出して変数に格納
    let (_, _, _, result2) = s;
    println!("合否 = {}", result2);
}