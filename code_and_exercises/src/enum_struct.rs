use clap::{Parser, Subcommand};
use anyhow::{Result};

#[derive(Parser, Debug)]
#[command(version, about= "A simple CLI utility", long_about= None)]
struct Cli {
    #[command(subcommand)]
    shape: Shapes,
}

#[derive(Subcommand, Debug)]
enum Shapes{
    Circle{
        #[arg(long)]
        radius: f64
    },

    Rectangle{
        #[arg(long)]
        height: f64,
        #[arg(long)]
        width: f64
    },

    Triangle{
        #[arg(long)]
        side1: f64,
        #[arg(long)]
        side2: f64,
        #[arg(long)]
        side3: f64
    },
}

#[derive(Debug)]
struct Circle{
    radius: f64
}

#[derive(Debug)]
struct Triangle{
    side1: f64,
    side2: f64,
    side3: f64
}

#[derive(Debug)]
struct Rectangle {
    height: f64,
    width: f64
}

trait Shape{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Shape for Circle{
    fn area(&self) -> f64{
        return std::f64::consts::PI * self.radius * self.radius;
    }
    fn perimeter(&self)-> f64{
        return 2.0 * std::f64::consts::PI * self.radius;
    }
}

impl Shape for Triangle{
    fn area(&self) -> f64{
        let s= (self.side1 + self.side2 + self.side3)/2.0;
        return (s* (s-self.side1)* (s- self.side2) * (s- self.side3)).sqrt();
    }
    fn perimeter(&self) -> f64{
        return self.side1+ self.side2+ self.side3;
    }
}

impl Shape for Rectangle{
    fn area(&self) -> f64{
        return self.height * self.width;
    }
    fn perimeter(&self) -> f64{
        return 2.0 * (self.height + self.width);
    }
}



fn main()-> Result<()>{
    let cli= Cli::parse();

    match &cli.shape {
        Shapes::Circle { radius } => {
            let circle= Circle{radius: *radius};
            println!("Area of the Circle: {}", circle.area());
            println!("Perimeter of the Circle: {}", circle.perimeter());
        },
        Shapes::Triangle { side1, side2, side3 } => {
            let triangle= Triangle{side1: *side1, side2: *side2, side3: *side3};
            println!("Area of Triangle: {}", triangle.area());
            println!("Perimeter of Triangle: {}", triangle.perimeter());
        },
        Shapes::Rectangle { height, width} => {
            let rectangle= Rectangle{height: *height, width: *width};
            println!("Area of Rectangle: {}", rectangle.area());
            println!("Perimeter of Rectangle: {}", rectangle.perimeter());
        }
    }
    Ok(())
}