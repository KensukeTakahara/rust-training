fn check1() {
  let t1 = (3, "birds".to_string());
  let mut b1 = Box::new(t1);
  (*b1).0 = 4;
  assert_eq!(*b1, (4, "birds".to_string()));

  // println!("{:?}", &t1);
}

pub fn check() {
  check1();
}
