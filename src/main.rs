
use std::any::Any;

use serde::{ Serialize, Deserialize };

use std::fs::File;

use std::fs;

use std::io::Write;


trait Shape: Any {
    fn as_any(&self) -> &dyn Any;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug, Serialize, Deserialize)]
struct Circle {
    radius: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct Rectangle {
    width: f64,
    height: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct Triangle {
    base: f64,
    height: f64,
}
//IMPLEMENTATIONS OF THE TRAIN IN THE STRUCTS
impl Shape for Circle {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn perimeter(&self) -> f64 {
        // Let's assume an equilateral triangle for simplicity
        3.0 * self.base
    }
}

enum ShapeType {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl ShapeType {
    fn area(&self) -> f64 {
        match self {
            ShapeType::Circle(c) => c.area(),
            ShapeType::Rectangle(r) => r.area(),
            ShapeType::Triangle(t) => t.area(),
        }
    }
    fn perimeter(&self) -> f64 {
        match self {
            ShapeType::Circle(c) => c.perimeter(),
            ShapeType::Rectangle(r) => r.perimeter(),
            ShapeType::Triangle(t) => t.perimeter(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ShapeWrapper {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle {base: f64, height: f64}
}

impl ShapeWrapper {
    fn into_shape(self) -> Box<dyn Shape> {
        match self {
            ShapeWrapper::Circle { radius } => Box::new(Circle { radius }),
            ShapeWrapper::Rectangle { width, height } => Box::new(Rectangle { width, height }),
            ShapeWrapper::Triangle { base,height } => Box::new(Triangle { base, height }),
        }
    }

    fn from_shape(shape: &dyn Shape) -> Option<ShapeWrapper> {
        if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
            Some(ShapeWrapper::Circle {
                radius: circle.radius,
            })
        } else if let Some(rect) = shape.as_any().downcast_ref::<Rectangle>() {
            Some(ShapeWrapper::Rectangle {
                width: rect.width,
                height: rect.height,
            })
        } else if let Some(tri) = shape.as_any().downcast_ref::<Triangle>() {
            Some(ShapeWrapper::Triangle {
                base: tri.base,
                height: tri.height,
            })
        }else {
            None
        }
    }
}

fn main() {

     let json_input = r#"
    [
        { "type": "Circle", "radius": 3.0 },
        { "type": "Rectangle", "width": 4.0, "height": 5.0 },
         { "type": "Triangle", "base": 4.0, "height": 3.0 }
    ]
    "#;

    let wrappers: Vec<ShapeWrapper> = serde_json::from_str(json_input).expect("Invalid JSON");
    let shapes: Vec<Box<dyn Shape>> = wrappers.into_iter().map(|w| w.into_shape()).collect();

 // Write shapes to file
    write_shapes_to_file(&shapes, "shapes.json");

    println!("Shapes saved to shapes.json");


    // Read back from file
    let loaded_shapes = match read_shapes_from_file("shapes.json")
    {
        Ok(val)=> val,
        Err(e)=> panic!(),
    };
    

    for shape in &loaded_shapes {
        println!("Loaded shape area: {:.2}", shape.area());
    }

//let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { radius: 3.0 }),Box::new(Rectangle{width:2.0,height:3.0})];

 for shape in &shapes {
        println!("Area: {:.2}", shape.area());
        println!("{:?}" ,ShapeWrapper::from_shape(shape.as_ref()));

        // Try to downcast to Circle
        if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
            println!(" -> This is a Circle with radius: {:.2}", circle.radius);
        } else if let Some(rect) = shape.as_any().downcast_ref::<Rectangle>() {
            println!(" -> This is a Rectangle with width: {:.2}, height: {:.2}", rect.width, rect.height);
        }
        else if let Some(tri) = shape.as_any().downcast_ref::<Triangle>() {
            println!(" -> This is a triangle with base: {:.2}, height: {:.2}", tri.base, tri.height);
     
        } else {
            println!(" -> Unknown shape.");
        }
    }
    
 // Convert trait objects back into ShapeWrapper for serialization
    let wrappers_for_output: Vec<ShapeWrapper> = shapes
        .iter()
        .filter_map(|s| ShapeWrapper::from_shape(s.as_ref()))
        .collect();

    let serialized_json = serde_json::to_string_pretty(&wrappers_for_output).unwrap();
    println!("\nSerialized back to JSON:\n{}", serialized_json);


    let shapes = vec![
        ShapeWrapper::Circle { radius: 3.0 },
        ShapeWrapper::Rectangle { width: 4.0, height: 5.0 },
        ShapeWrapper::Triangle { base: 6.0, height: 2.0 }
    ];

    let json = serde_json::to_string_pretty(&shapes).unwrap();
    println!("Serialise to Json {}", json);

    // with Box containing anything that implements shape (polymorphism e.g.)
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
        Box::new(Triangle { base: 6.0, height: 2.0 })
    ];

    for shape in shapes.iter() {
        println!("Area: {:.2}, Perimeter: {:.2}", shape.area(), shape.perimeter());
    }

    //with enum

    let shapes: Vec<ShapeType> = vec![
        ShapeType::Circle(Circle { radius: 3.0 }),
        ShapeType::Rectangle(Rectangle { width: 4.0, height: 5.0 }),
        ShapeType::Triangle(Triangle { base: 6.0, height: 2.0 })
    ];

    for shape in shapes.iter() {
        println!("Area: {:.2}, Perimeter: {:.2}", shape.area(), shape.perimeter());
    }
}

fn write_shapes_to_file(shapes: &[Box<dyn Shape>], filename: &str) -> std::io::Result<()> {
    let wrappers: Vec<ShapeWrapper> = shapes
        .iter()
        .filter_map(|s| ShapeWrapper::from_shape(s.as_ref()))
        .collect();

    let json = serde_json::to_string_pretty(&wrappers)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
fn read_shapes_from_file(filename: &str) -> std::io::Result<Vec<Box<dyn Shape>>> {
    let contents = fs::read_to_string(filename)?;
    let wrappers: Vec<ShapeWrapper> = serde_json::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(wrappers.into_iter().map(|w| w.into_shape()).collect())
}