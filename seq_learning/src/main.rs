use std::collections::{HashMap, HashSet};
use std::ops::Add;
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum RobotError {
//     #[error("sensor '{name}' not responding after {timeout_ms}ms")]
//     SensorTimeout { name: String, timeout_ms: u64 },

//     #[error("invalid velocity: {0} (must be in [-1.0, 1.0])")]
//     InvalidVelocity(f64),

//     #[error("IO error")]
//     Io(#[from] std::io::Error), // #[from] implements From<std::io::Error> for RobotError

//     #[error("ROS2 error: {0}")]
//     Ros(#[from] rclrs::RclrsError),
// }
#[derive(Debug)]
struct User {
    username: String,
    age: u8,
    is_alive: bool,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function (no self) — called as Point::origin()
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    // Method — borrows self immutably
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Method — borrows self mutably
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    // Method — consumes self (ownership transfer)
    fn into_tuple(self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn into_tuple_ok(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn quadrant(&self) -> (bool, bool) {
        (self.x >= 0.0, self.y >= 0.0)
    }

    fn symmetric_point(d: f64) -> Self {
        Self { x: d, y: d }
    }
}

#[derive(Debug)]
enum Student {
    Cracked { age: u8, ranking: u8 },
    Terrible,
    Intern { duration: u8, school: String },
}

impl Student {
    fn ability(&self) -> u8 {
        match self {
            // Student::Cracked{age, ranking} => 1000.0 / powf64(2.0, *ranking as f64) as u8,
            Student::Cracked { age, ranking } => {
                if 100 < age + ranking {
                    0
                } else {
                    100 - age - ranking
                }
            }
            Student::Terrible => 0,
            Student::Intern { duration, school } => match school.as_str() {
                "NUSH" => 50 + duration,
                _ => 0 + duration,
            },
        }
    }
}

enum Shape {
    Circle(f64),                         // radius
    Rectangle(f64, f64),                 // width, height
    Triangle { base: f64, height: f64 }, // named fields
    Point,                               // no data
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
            Shape::Point => 0.0,
        }
    }
}

trait Inspect {
    fn inspect(&self);
}

// Implement it for i32
impl Inspect for i32 {
    fn inspect(&self) {
        println!("Value: {}", self);
    }
}

fn process_item<T: Inspect>(item: &T) {
    item.inspect();
}

fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn meta_fn1(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
fn meta_fn2(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x) // narrower than meta_fn3/4
}
fn meta_fn3(f: impl FnOnce(i32) -> i32, x: i32) -> i32 {
    f(x) // broadest
}
fn meta_fn4(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    f(x)
}
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! vec_str {
    ($($s:expr), *) => {
        {
            let mut v = Vec::new();
            $(
                v.push(String::from($s));
            )*
            v
        }
    };
}

macro_rules! vec_string {
    ($($s:expr), *) => {
        vec![$($s.to_string()), *]
    };
}

#[allow(dead_code, unused_variables)]
fn main() {
    println!("Hello, world!");
    println!("{}", format!("{} + {} = {}", 1, 2, 1 + 2));

    let mut u1: User = User {
        username: String::from("Bob"),
        age: 99,
        is_alive: true,
    };

    println!("U1: {:?}", u1);

    let u2: &mut User = &mut u1;

    u2.age += 1;
    u2.is_alive = false;
    u2.username.push_str(" Alan");
    u2.username += " Smith";
    u2.username = u2.username.clone().add(" Jr.");

    println!("U2: {:?}", u2);
    println!("U1: {:?}", u1);
    drop(u1);

    let mut p = Point { x: 3.0, y: 4.0 };
    println!("Point: {:?}", p); // Point { x: 3.0, y: 4.0 }
    println!("{}", p.distance_from_origin()); // 5.0
    println!("Quadrant: {:?}", p.quadrant()); // (true, true)
    p.translate(10.0, 0.0);
    let (x2, y2) = p.into_tuple_ok();
    println!("Point: ({:1.2}, {})", x2, y2); // (4.0, 4.0)
    println!("Point: {:?}", p); // Point { x: 3.0, y: 4.0 }
    let (x, y) = p.into_tuple();
    println!("Point: ({}, {})", x, y); // (4.0, 4.0)
    // p is no longer valid

    let zane: Student = Student::Cracked {
        age: 20,
        ranking: 1,
    };
    let terry: Student = Student::Terrible;
    let ethan: Student = Student::Intern {
        duration: 3,
        school: String::from("NUSH"),
    };
    let evan: Student = Student::Intern {
        duration: 3,
        school: String::from("International School"),
    };

    println!("Zane's ability: {}", zane.ability());
    println!("Terry's ability: {}", terry.ability());
    println!("Ethan's ability: {}", ethan.ability());
    println!("Evan's ability: {}", evan.ability());
    println!("Evan's real power: {}", (evan.ability() as f64).powi(5));
    println!("Evan's real power: {}", (evan.ability() as u32).pow(5));

    println!("{:?}", evan);
    println!("{:#?}", evan);

    let number: i32 = 42;
    number.inspect();
    process_item(&number);

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("---");
    v.iter().for_each(|item| item.inspect());
    println!("---");
    for item in &mut v {
        *item += 1;
        item.inspect();
    }
    println!("---");
    for item in v.iter() {
        item.inspect();
    }
    println!("---");
    for &item in &v {
        item.inspect();
    }
    println!("---");
    for &item in v.iter() {
        item.inspect();
    }
    println!("---");
    // for &item in v { // already i32 type, cannot be assigned as &(unknown)
    //     item.inspect();
    // }
    println!("---");
    for item in v {
        // consumes
        item.inspect();
    }
    println!("---");

    let double = make_multiplier(2);
    println!("{}", double(5)); // Prints 10
    println!("{}", make_multiplier(2)(5)); // Prints 10

    let s: String = String::from("Hello, world!");
    let s2: String = s.trim().to_string();
    let s3: &str = "Hello, world!";
    let s4: String = s3.trim().to_string();

    let s5 = s4.parse::<i32>();
    let vv: Vec<_> = (0..10).collect::<Vec<i32>>();
    let v4: Vec<i32> = core::iter::Iterator::collect::<Vec<i32>>((0..10).map(|x| x * 2));
    let i5: i32 = str::parse::<i32>("Hello, world!").unwrap_or_else(|_| 0);

    let sx = s3.split(',').next().unwrap_or("Default value");
    let sy = String::from("Hello")
        .split(',')
        .next()
        .unwrap_or("Default value")
        .to_string(); // required else dangling ref
    println!("sx: {}", sx);
    println!("sy: {}", sy);

    let v1: Vec<i32> = (0..10).collect();
    let v2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let v3: Vec<i32> = vec![3; 5];
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);
    println!("v1 slice: {:?}", &v1[2..4]);

    let mut set1: HashSet<i32> = HashSet::new();
    let mut set2: HashSet<i32> = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);

    let union = &set1 | &set2;
    let intersection = &set1 & &set2;
    let tmp = &set1;
    println!("Union: {:?}", union);
    println!("Intersection: {:?}", intersection);
    println!("Tmp: {:?}", tmp);

    let set3 = [1, 2, 3].into_iter().collect::<HashSet<_>>();
    let set4: HashSet<i32> = [1, 2, 3].into_iter().collect();
    println!("Set3: {:?}", set3);
    println!("Set4: {:?}", set4);

    let map_borrow: HashMap<_, _> = set1.iter().zip(set2.iter()).collect();
    println!("Map borrow: {:#?}", map_borrow);
    // above println has to be before the next line, else set1 and set2 will be moved into map_own & map_borrow is invalidated
    let map_own: HashMap<_, _> = set1.into_iter().zip(set2.into_iter()).collect();
    println!("Map own: {:#?}", map_own);
    // IntoIterator::into_iter

    let ss: String = "hello".into();
    println!("ss: {}", ss);

    let a = vec![1, 2, 3];
    let b = vec!["a", "b", "c"];
    let pairs: Vec<_> = a.iter().zip(b.iter()).collect(); // [(1, "a"), (2, "b"), (3, "c")]
    for (i, x) in pairs.iter().enumerate() {
        println!("{}: {:?}", i, x);
    }

    let c1 = vec![1, 2, 3];
    let c2 = vec![4, 5, 6];
    let combined: Vec<_> = c1.into_iter().chain(c2.into_iter()).collect();

    let v9: Vec<i32> = vec!["1", "x", "3"]
        .iter()
        .flat_map(|s| s.parse::<i32>().ok())
        .collect();
    let v8: Vec<Option<i32>> = vec!["1", "x", "3"]
        .iter()
        .map(|s| s.parse::<i32>().ok())
        .collect();
    println!("v9: {:?}", v9);
    println!("v8: {:?}", v8);

    println!("meta_fn1: {}", meta_fn1(|x| x + 1, 5));
    println!("meta_fn2: {}", meta_fn2(|x| x + 1, 5));
    println!("meta_fn3: {}", meta_fn3(|x| x + 1, 5));
    println!("meta_fn4: {}", meta_fn4(|x| x + 1, 5));

    let y = 10;
    meta_fn1(|x| x + 1, 5); // OK
    // meta_fn1(|x| x + y, 5); // ERR, captures y, fat ptr
    meta_fn2(|x| x + 1, 5); // OK
    meta_fn2(|x| x + y, 5); // OK

    say_hello!();
    let hello = stringify!(Hello, world!);
    println!("hello: {}", hello);
    println!("file: {}", file!());

    println!("PATH: {}", env!("PATH"));
    for var in [
        "DOESNOTEXIST",
        "HOME",
        "PATH",
        "SHELL",
        "USER",
        "LOGNAME",
        "PWD",
    ] {
        println!(
            "{}: {}",
            var,
            std::env::var(var).unwrap_or_else(|_| "Not found".to_string())
        );
    }

    println!("vec_str: {:?}", vec_str!["a", "b", "c"]);
    println!("vec_string: {:?}", vec_string!["a", "b", "c"]);
}
