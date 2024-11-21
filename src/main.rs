use core::fmt;
use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
    
}
pub struct Circle {
    radius:f64,
}

impl Circle {
    /// Constructor
    pub fn try_new(arg1:f64) ->Result<Self, &'static str> {
        if arg1 >0.0 {
            Ok(Self { radius: arg1 })
        } else {
            Err("Circle Radius must be greater than equal to 0.")
        }
    }
    pub fn set_r(&mut self,r:&f64) -> Result<(), &'static str> {
        if *r>0.0 {
            self.radius = *r;
            Ok(())
        } else {
            Err("Circle radius muct be greater than 0")
        }
    }
  
    pub fn get_r(&self)->f64 {
        return self.radius;
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}


impl fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = ".........................................\n".to_string();
        out.push_str("Printing Circle \n");
        out.push_str(&format!(
            "r: {}, area: {}, circumference:{}\n",
            self.get_r(),
            self.area(),
            self.circumference()
        ));
        out.push_str(".........................................\n");
        write!(f, "{}", out)
    }
}

fn circle_example() {
    let r_in:f64 = 17.0;
    let mut circle = Circle::try_new(r_in).unwrap();
    print!("Circle {}",circle);

    let new_r_in:f64 = 15.0;
    let res = circle.set_r(&new_r_in);
    assert!(res.is_ok());
    print!("Circle {}",circle);

    let new_r_in = -15.0;
    let res = circle.set_r(&new_r_in);

    assert_eq!(res.err(), Some("Circle radius muct be greater than 0"));
    

}
pub fn main() {
    circle_example();
}