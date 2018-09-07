use std::io::*;
use std::str::FromStr;
//01.標準入力(n) n:i64
//let n = read_i();
fn read_i() -> i64 {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let token = stdin
        .by_ref().bytes().map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .fold(0, |a, x| (x as u8 - b'0') as i64 + a * 10);
    token
}
//02.標準入力(s) s:Vec<char>
//let s = read_s();
fn read_s() -> Vec<char> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token = stdin
        .bytes().map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !(*c=='\r'||*c=='\n')).collect();
    token
}
use std::io::*;
use std::str::FromStr;
//03.標準入力(n) n:FromStr
//let n:u32=read();
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !(*c=='\r'||*c=='\n'))
        .collect();
    token.parse().ok().expect("failed to parse token")
}
//04.標準入力[a(1)...a(n)]
//let a:Vec<i64>=read_a();
fn read_a<T: FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
//05.標準入力(a,b)*n
//let mut a=Vec::new();
//let mut b=Vec::new();
//for _ in 0..n{
//    let tl:Vec<i64>=read_a();
//    a.push(tl[0]);
//    b.push(tl[1]);
//}


//06.最小値・最大値
//let (min, Max) = find_min_max(&a);
use std::cmp;
fn find_min_max(x: &[i64]) -> (i64, i64) {
    let mut min: i64 = x[0];
    let mut max: i64 = x[0];
    let mut x_iter = x.iter();
    while let Some(i) = x_iter.next() {
        min = cmp::min(*i, min);
        max = cmp::max(*i, max);
    }
    return (min, max)
}
//配列の全要素を色々する
//for i in 0..n{b.push(a[i as usize]-i-1);}
//この場合、n:i64なため　i:i64
//もしfor i in 0..100の場合
//i:usizeになる
//a[i]+i とするとa[i]:i64, i:usizeより大だめ

//配列の長さ
//let n = s.len();

//abc..xyzかどうか
//if c < 'a' || 'z' < c
