use proconio::*;

#[fastout]
fn main() {
    input! {
    h: usize,
    w: usize,
    a: [[u32;w]; h],
  }
  
  let row = a.iter().map(|a| a.iter().sum::<u32>()).collect::<Vec<_>>();
  let col = (0..w)
            .map(|k| a.iter().fold(0, |s,a| s+a[k]))
            .collect::<Vec<_>>();

  let mut s = String::new();
  for (row,a) in row.iter().zip(a) {
    for (col, a) in col.iter().zip(a){
      s.push_str(&format!("{} ", *row+ *col-a));
    }
    s.pop();
    s.push('\n');
  }
  print!("{}",s);
}
