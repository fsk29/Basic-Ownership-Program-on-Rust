fn main() {
//    let message = "hello world";
//    let x: i32 = 42;
//    let pi : f64 =3.14;
//    let is_rust_fun: bool = true;
//    let letter_a : char ='a';
   fn add( x: i32, y: i32) -> i32{
        x+y 
   }
   let result:i32 = add(32, 32);
   result.to_string();
   println!("{}", result);


   if result < 0 {
    println!("result is negative");
   }
   else {
       println!("result is positive");
   }


   let mut i = 1;


while i <= 5 {
println!("{}", i);
      i += 1;
}


}
