use proconio::*;

fn main() {
  input!{
      n:usize,
    }
  
  for s in  1..(1<<n) {
    let mut cnt = 0;
    for i in 0..n {
      cnt += if (s>>i)&1==1 {1} else{-1}; 
      if cnt < 0 {break;}
    } 
    if cnt ==0 {
      for i in 0..n {
        print!("{}",if (s>> (n-i-1)) & 1 ==0 {"("}else{")"});
      }
      println!("");
    }
  }
}

