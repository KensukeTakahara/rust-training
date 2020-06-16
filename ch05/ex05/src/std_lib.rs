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

pub fn check() {
  check1();
  check2();
  check3();
  check4();
}
