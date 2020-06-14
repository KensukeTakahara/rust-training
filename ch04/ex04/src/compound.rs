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

fn print_info(name: &str, s1: &[char]) {
  println!(
    "  {:9} - {}, {:?}, {:?}, {:?}",
    name,
    s1.len(),
    s1.first(),
    s1[1],
    s1.last()
  );
}

fn check9() {
  let a1 = ['a', 'b', 'c', 'd'];
  println!("a1: {:?}", a1);

  print_info("&a1[..]", &a1[..]);
  print_info("&a1", &a1);
  print_info("&a1[1..3]", &a1[1..3]);

  let v1 = vec!['e', 'f', 'g', 'h'];
  println!("\nv1: {:?}", v1);

  print_info("&v1[..]", &v1[..]);
  print_info("&v1", &v1);
  print_info("&v1[1..3]", &v1[1..3]);
}

fn check10() {
  let mut a1 = [5, 4, 3, 2];
  let s1 = &mut a1[1..3];
  s1[0] = 6;
  s1[1] *= 10;
  s1.swap(0, 1);
  assert_eq!(s1, [30, 6]);

  assert_eq!(a1, [5, 30, 6, 2]);
}

fn check11() {
  let a2: [i32; 0] = [];
  let s2 = &a2;
  assert!(s2.is_empty());
  assert_eq!(s2.len(), 0);
  assert_eq!(s2.first(), None);

  let a3 = ["zero", "one", "two", "three", "four"];
  let s3 = &a3[1..4];
  assert!(!s3.is_empty());
  assert_eq!(s3.len(), 3);
  assert_eq!(s3.first(), Some(&"one"));

  assert_eq!(s3[1], "two");
  assert_eq!(s3.get(1), Some(&"two"));
  assert_eq!(s3.get(3), None);

  assert!(s3.contains(&"two"));
  assert!(s3.starts_with(&["one", "two"]));
  assert!(s3.ends_with(&["two", "three"]));
}

fn check12() {
  let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];
  &mut a4[2..5].sort();
  assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

  let (s4a, s4b) = &mut a4.split_at_mut(5);
  s4a.reverse();
  assert_eq!(s4a, &[8, 2, 0, 4, 6]);

  s4b.sort_unstable();
  assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);
}

fn check13() {
  let s1 = "abc1";
  let s2 = "abc2";
  assert!(s1 < s2);
  assert!(s1 != s2);
}

fn check14() {
  let fruits = "あかりんご, あおりんご\nラスベリー, ブラックベリー";

  let mut lines = fruits.lines();
  let apple_line = lines.next();
  assert_eq!(apple_line, Some("あかりんご, あおりんご"));
  assert_eq!(lines.next(), Some("ラスベリー, ブラックベリー"));
  assert_eq!(lines.next(), None);

  if let Some(apples) = apple_line {
    assert!(apples.starts_with("あか"));
    assert!(apples.contains("りんご"));
    assert_eq!(apples.find("あお"), Some(17));

    let mut apple_iter = apples.split(",");
    assert_eq!(apple_iter.next(), Some("あかりんご"));

    let green = apple_iter.next();
    assert_eq!(green, Some(" あおりんご"));
    assert_eq!(green.map(str::trim), Some("あおりんご"));

    assert_eq!(apple_iter.next(), None);
  } else {
    unreachable!();
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
  check9();
  check10();
  check11();
  check12();
  check13();
  check14();
}
