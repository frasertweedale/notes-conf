pub mod ex1 {

pub struct ErrNegative;

fn foo(input: Option<i32>) -> Option<i32> {
  let x = input?;
  if x < 0 { None } else { Some(x) }
}

pub fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
  foo(input).ok_or(ErrNegative)
}

}
