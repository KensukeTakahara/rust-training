type UserName = String;
type Id = i64;
type TimeStamp = i64;
type User = (Id, UserName, TimeStamp);

fn new_user(name: UserName, id: Id, created: TimeStamp) -> User {
  (id, name, created)
}

fn check1() {
  let id = 400;
  let now = 4567890123;
  let user = new_user(String::from("mika"), id, now);
  assert_eq!(user.0, id);
}

struct Polygon {
  vertexes: Vec<(i32, i32)>,
  stroke_width: u8,
  fill: (u8, u8, u8),
}

impl Default for Polygon {
  fn default() -> Self {
    Self {
      stroke_width: 1,
      vertexes: Default::default(),
      fill: Default::default(),
    }
  }
}

fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
  let stroke_width = 1;
  let fill = (255, 255, 255);
  Polygon {
    vertexes,
    stroke_width,
    fill,
  }
}

fn check2() {
  let triangle = Polygon {
    vertexes: vec![(0, 0), (3, 0), (2, 2)],
    fill: (255, 255, 255),
    stroke_width: 1,
  };

  let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);
  assert_eq!(triangle.fill, quadrangle.fill);
  assert_eq!(triangle.stroke_width, quadrangle.stroke_width);

  assert_eq!(triangle.vertexes[0], (0, 0));
  assert_eq!(triangle.vertexes.len(), 3);

  let Polygon {
    vertexes: quad_vx, ..
  } = quadrangle;
  assert_eq!(4, quad_vx.len());

  let Polygon { fill, .. } = quadrangle;
  assert_eq!((255, 255, 255), fill);

  let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
  assert_eq!(polygon.vertexes.len(), 2);
  polygon.vertexes.push((2, 8));
  assert_eq!(polygon.vertexes.len(), 3);
}

fn check3() {
  let triangle1 = Polygon {
    vertexes: vec![(0, 0), (3, 0), (2, 2)],
    fill: (255, 255, 255),
    stroke_width: 5,
  };

  let triangle2 = Polygon {
    vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
    ..triangle1
  };

  assert_eq!(triangle1.stroke_width, triangle2.stroke_width);
}

fn check4() {
  let polygon1: Polygon = Default::default();
  assert_eq!(polygon1.stroke_width, 1);
  assert_eq!(polygon1.fill, (0, 0, 0));
  assert_eq!(polygon1.vertexes, []);
}

struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

fn check5() {
  let vx0 = Vertex(0, 0);
  let vx1 = Vertex(3, 0);
  let triangle = Triangle(vx0, vx1, Vertex(2, 2));

  assert_eq!((triangle.1).0, 3);
}

#[derive(Debug, PartialEq)]
struct UniqueValue;

fn check6() {
  let uv1 = UniqueValue;
  let uv2 = UniqueValue;
  assert_eq!(uv1, uv2);
}

pub fn check() {
  check1();
  check2();
  check3();
  check4();
  check5();
  check6();
}
