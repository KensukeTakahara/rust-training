fn check1() {
  let t1 = (88, true);
  assert_eq!(t1.0, 88);
  assert_eq!(t1.1, true);
}

fn check2() {
  let mut t1 = (88, true);
  t1.0 += 100;
  assert_eq!(t1, (188, true));
}

fn check3() {
  let (n1, b1) = (88, true);
  assert_eq!(n1, 88);
  assert_eq!(b1, true);

  let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
  assert_eq!(x1, 0);
  assert_eq!(y1, 5);
  assert_eq!(x2, 10);
  assert_eq!(y2, -1);
}

fn check4() {
  let mut t1 = ((0, 5), (10, -1));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;
  *x1_ptr += 3;
  *y1_ptr *= -1;
  assert_eq!(t1, ((3, -5), (10, -1)));
}

fn check5() {
  // let a1 = [false, true, false];
  let a2 = [0.0, -1.0, 1.0, 0.5];
  assert_eq!(a2.len(), 4);

  let a3 = [0; 100];
  assert_eq!(a3.len(), 100);

  // let a4 = [['a', 'b'], ['c', 'd']];
}

fn check6() {
  let size = 100;
  let mut v1 = vec![0; size];
  assert_eq!(v1.len(), size);

  v1.push(1);
  assert_eq!(v1.len(), 101);
  assert_eq!(v1.pop(), Some(1));
  assert_eq!(v1.len(), 100);
}

fn check7() {
  let array3 = [0, 1];
  assert_eq!(array3.get(1), Some(&1));
  assert_eq!(array3.get(2), None);
}

fn check8() {
  let array4 = ['a'; 50];

  for ch in array4.iter() {
    print!("{}, ", ch);
  }
  print!("\n");

  let mut array5 = [1; 50];

  for n in array5.iter_mut() {
    *n *= 2;
  }
}

pub fn check() {
  check1();
  check2();
  check3();
  check4();
  check5();
  check6();
  check7();
  check8();
}
