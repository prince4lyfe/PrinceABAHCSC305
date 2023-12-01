fn main() {
  // Fixed-size array (type signature is superfluous).
  let xs: [i64; 5] = [1, 2, 3, 4, 5]; 


  fn multiplier (xs: &[f64])-> f64 {

    let i = 0;
    let mut product:f64 = 1;
    while i < xs.len(){
      product = product*arr[i];
      i+=1;
    }

  return product;
  }

}