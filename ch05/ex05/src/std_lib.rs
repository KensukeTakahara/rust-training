fn check1() {
  let t1 = (3, "birds".to_string());
  let mut b1 = Box::new(t1);
  (*b1).0 = 4;
  assert_eq!(*b1, (4, "birds".to_string()));

  // println!("{:?}", &t1);
}

fn check2() {
  // let v1 = vec![false, true, false];
  let v2 = vec![0.0, -1.0, 1.0, 0.5];

  assert_eq!(v2.len(), 4);

  let v3 = vec![0; 100];
  assert_eq!(v3.len(), 100);

  let v4 = vec![vec!['a', 'b', 'c'], vec!['d']];
  assert_eq!(v4[0][0], 'a');

  // let v5 = vec![false, 'a'];
}

fn check3() {
  let mut v6 = vec!['a', 'b', 'c'];
  v6.push('d');
  v6.push('e');
  assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);

  assert_eq!(v6.pop(), Some('e'));
  v6.insert(1, 'f');
  assert_eq!(v6.remove(2), 'b');
  assert_eq!(v6, ['a', 'f', 'c', 'd']);

  let mut v7 = vec!['g', 'h'];
  v6.append(&mut v7);
  assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
  assert_eq!(v7, []);

  let a8 = ['i', 'j'];
  v6.extend_from_slice(&a8);
  assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
  assert_eq!(a8, ['i', 'j']);
}

fn check4() {
  let mut v1 = vec![0, 1, 2, 3];
  v1.push(4);
  println!("v1 len: {}, capacity:{}", v1.len(), v1.capacity());

  let s1 = v1.into_boxed_slice();
  let v2 = s1.into_vec();
  println!("v2 len: {}, capacity:{}", v2.len(), v2.capacity());
}

use std::collections::HashMap;

fn check5() {
  let mut m1 = HashMap::new();

  m1.insert("a", 1);
  m1.insert("b", 5);
  assert_eq!(m1.len(), 2);

  assert_eq!(m1.get("b"), Some(&5));
  assert_eq!(m1.get("c"), None);

  let d = m1.entry("d").or_insert(0);
  *d += 7;
  assert_eq!(m1.get("d"), Some(&7));

  let m2 = vec![("a", 1), ("b", 3)]
    .into_iter()
    .collect::<HashMap<_, _>>();
  assert_eq!(m2.len(), 2);
  assert_eq!(m2.get("a"), Some(&1));
}

fn check6() {
  let mut s1 = "ラズベリー".to_string();
  let mut s2 = String::from("ブラックベリー");

  let s3 = "ストロベリー".to_owned();

  s1.push_str("タルト");
  assert_eq!(s1, "ラズベリータルト");

  s2.push('と');
  s2.push_str(&s3);
  assert_eq!(s2, "ブラックベリーとストロベリー");
}

fn check7() {
  let i = 42;
  assert_eq!(i.to_string(), "42");

  let f = 4.3 + 0.1;
  assert_eq!(f.to_string(), "4.3999999999999995");
  assert_eq!(format!("{:.2}", f), "4.40");

  let t = (1, "ABC");
  assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);
}

fn check8() {
  let s1 = "42";
  assert_eq!(s1.parse::<i32>(), Ok(42));

  let s2 = "abc";
  let r2: Result<f64, _> = s2.parse();
  assert!(r2.is_err());
  println!("{:?}", r2);
}

fn check9() {
  let cs = ['t', 'r', 'u', 's', 't'];
  assert_eq!(cs.iter().collect::<String>(), "trust");
  assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

  let bad_utf8: [u8; 7] = [b'a', 0xf0, 0x90, 0x80, 0xe3, 0x81, 0x82];
  let s = String::from_utf8_lossy(&bad_utf8);
  assert_eq!(s, "a\u{fffd}あ");
}

fn f1(name: &str) -> String {
  format!("Hello, {}!", name)
}

fn check10() {
  let a = ['a', 'b', 'c', 'd', 'e'];

  assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
  assert_eq!(a[..3], ['a', 'b', 'c']);
  assert_eq!(a[..=3], ['a', 'b', 'c', 'd']);
  assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
  assert_eq!(a[1..3], ['b', 'c']);
  assert_eq!(a[1..=3], ['b', 'c', 'd']);

  assert_eq!(.., std::ops::RangeFull);
  assert_eq!(..3, std::ops::RangeTo { end: 3 });
  assert_eq!(..=3, std::ops::RangeToInclusive { end: 3 });
  assert_eq!(1.., std::ops::RangeFrom { start: 1 });
  assert_eq!(1..3, std::ops::Range { start: 1, end: 3 });
  assert_eq!(1..=3, std::ops::RangeInclusive::new(1, 3));
}

fn check11() {
  let a1 = ['a', 'b', 'c', 'd'];
  assert_eq!(a1.get(0), Some(&'a'));
  assert_eq!(a1.get(4), None);

  let mut o1 = Some(10);
  match o1 {
    Some(s) => assert_eq!(s, 10),
    None => unreachable!(),
  }

  o1 = Some(20);
  if let Some(s) = o1 {
    assert_eq!(s, 20);
  }
}

fn check12() {
  let mut o2 = Some(String::from("Hello"));
  assert_eq!(o2.unwrap(), "Hello");

  o2 = None;
  // o2.unwrap();
  assert_eq!(
    o2.unwrap_or_else(|| String::from("o2 is none")),
    "o2 is none"
  );
}

fn check13() {
  let mut o3 = Some(25);
  assert_eq!(o3.map(|n| n * 10), Some(250));

  o3 = None;
  assert_eq!(o3.map(|n| n * 10), None);

  o3 = Some(10);
  assert_eq!(
    o3.map(|n| n * 10)
      .and_then(|n| if n >= 200 { Some(n) } else { None }),
    None
  );
}

fn add_elems(s: &[i32]) -> Option<i32> {
  let s0 = s.get(0)?;
  let s3 = s.get(3)?;
  Some(s0 + s3)
}

fn check14() {
  assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
  assert_eq!(add_elems(&[7, 11]), None);
}

fn check15() {
  assert_eq!("10".parse::<i32>(), Ok(10));
  let res0 = "a".parse::<i32>();
  assert!(res0.is_err());
  println!("{:?}", res0);
}

fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
  let s0 = s0.parse::<i32>()?;
  let s1 = s1.parse::<i32>()?;
  Ok(s0 + s1)
}

fn check16() {
  assert_eq!(add0("3", "127"), Ok(3 + 127));
  assert!(add0("3", "abc").is_err());
}

fn add1(s0: &str, s1: &str) -> Result<i32, String> {
  let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
  let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数ではありません")?;
  Ok(s0 + s1)
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
  println!("{}", f1("World"));
  check10();
  check11();
  check12();
  check13();
  check14();
  check15();
  check16();
  assert_eq!(add1("3", "abc"), Err("s1が整数ではありません".to_string()));
}
