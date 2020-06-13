fn hello() {
  println!("Hello");
}

fn check1() {
  let ret = hello();
  assert_eq!(ret, ());

  assert_eq!(std::mem::size_of::<()>(), 0);
}

fn check2() {
  let n1 = 10_000;
  let n2 = 0u8;
  let n3 = -100_isize;

  println!("{}, {}, {}", n1, n2, n3);

  let n6 = b'A';
  assert_eq!(n6, 65u8);
}

// fn check3() {
//   let n1 = std::u8::MAX;
//   let n2 = 1u8;
//   let n3 = n1 + n2;

//   println!("{}", n3);
// }

fn check4() {
  let n1 = 200u8;
  let n2 = 3u8;

  assert_eq!(n1.checked_mul(n2), None);
  assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
  assert_eq!(n1.wrapping_mul(n2), 88);
  assert_eq!(n1.overflowing_mul(n2), (88, true));
}

fn check5() {
  let c1 = 'A';
  let c2 = 'a';
  assert!(c1 < c2);
  assert!(c1.is_uppercase());

  let c3 = '0';
  assert!(c3.is_digit(10));

  assert_eq!(std::mem::size_of::<char>(), 4);
}

fn f1(mut n: u32) {
  println!("f1 before: n={}", n);
  n = 1;
  println!("f1 after: n={}", n);
}

fn f2(n_ptr: &mut u32) {
  println!("f2: n_ptr={:p}", n_ptr);
  *n_ptr = 2;
  println!("f2: *n_ptr={}", *n_ptr);
}

fn check6() {
  let mut n = 0;
  println!("main n={}", n);

  f1(n);
  println!("main n={}", n);
  f2(&mut n);
  println!("main n={}", n);
}

fn check7() {
  let c1 = 'A';
  let c1_ptr: *const char = &c1;
  assert_eq!(unsafe { *c1_ptr }, 'A');

  let mut n1 = 0;
  let n1_ptr: *mut i32 = &mut n1;
  assert_eq!(unsafe { *n1_ptr }, 0);

  unsafe {
    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1_000);
  }
}

fn double(n: i32) -> i32 {
  n + n
}

fn abs(n: i32) -> i32 {
  if n >= 0 {
    n
  } else {
    -n
  }
}

fn check8() {
  let mut f: fn(i32) -> i32 = double;
  assert_eq!(f(-42), -84);

  f = abs;
  assert_eq!(f(-42), 42);

  assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());
}

fn check9() {
  let x = 4;
  let adder = |n| n + x;
  assert_eq!(adder(2), 4 + 2);

  let mut state = false;
  let mut flipflop = || {
    state = !state;
    state
  };

  assert!(flipflop());
  assert!(!flipflop());
  assert!(flipflop());

  assert!(state);
}

pub fn check() {
  check1();
  check2();
  // check3();
  check4();
  check5();
  check6();
  check7();
  check8();
  check9();
}
