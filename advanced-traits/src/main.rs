use std::ops::Add;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point{
     type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters{
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

impl Display for Millimeters{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mm", self.0)
    }
}


trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);

    fn assosiate();
}

struct Human;

impl Human{
    fn fly(&self){
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human{
    fn fly(&self){
        println!("This is your captain speaking");
    }
}

impl Wizard for Human{
    fn fly(&self){
        println!("Up!");
    }

    fn assosiate(){
        println!("fly");
    }
}


trait OutlinePrint: Display{
    fn outline_print(&self){
        let output = self.to_string(); // to_string is a trait method that converts the type to a String , Display trait
        let len = output.len();
    }
}

fn main(){
    // let p1 = Point { x: 1, y: 0 };
    // let p2 = Point { x: 2, y: 3 };
    // let p3 = p1 + p2;
    // assert_eq!(p3, Point { x: 3, y: 3 });
    // println!("{:?}", p3);


    // let mm = Millimeters(1000);
    // let m = Meters(1);

    // let mm_plus_m = mm + m;
    // println!("{}", mm_plus_m);


    let person = Human;

    person.fly();

    Pilot::fly(&person);

    Wizard::fly(&person);

    <Human as Wizard>::assosiate(); // syntax for calling associated function
}