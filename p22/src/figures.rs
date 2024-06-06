#[derive(Debug, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
pub struct Circle {
    center: Point,
    radius: f32,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    points: [Point; 3],
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    points: [Point; 4],
}

#[derive(Debug, Clone)]
pub enum Shape {
    Point,
    Circle,
    Triangle,
    Rectangle,
}

pub trait Distance {
    fn distance_between_2_points(&self, points: [Point; 2]) -> f32;
}

pub trait RectangleDimensions {
    fn get_width(&self) -> i32;
    fn get_length(&self) -> i32;
}

pub trait TriangleHeight {
    fn get_height(&self) -> i32;
}

impl TriangleHeight for Triangle {
    fn get_height(&self) -> i32 {
        todo!()
    }
}

pub trait CirclePerimeter {
    fn get_circle_perimeter(&self) -> f32;
}

impl CirclePerimeter for Circle {
    fn get_circle_perimeter(&self) -> f32 {
        todo!()
    }
}

pub trait Area {
    fn area(&self, shape: Shape) -> f32;
}

impl Area for Shape {
    fn area(&self, _shape: Shape) -> f32 {
        todo!()
    }
}

pub trait Perimeter {
    fn perimter(&self, item: &impl Distance, shape: Shape) -> f32;
    //println!("Breaking news! {}", item.distance_between_2_points(points));
}

impl Perimeter for Shape {
    fn perimter(&self, item: &impl Distance, shape: Shape) -> f32 {
        let _ = item;
        let _ = shape;
        todo!()
    }
}

pub trait CompareShapeArea {
    fn get_shape_with_greater_area(&self, shapes: &[Shape]) -> &[Shape];
}

impl Distance for Point {
    fn distance_between_2_points(&self, points: [Point; 2]) -> f32 {
        let point0: Point = points[0].clone();
        let point1: Point = points[1].clone();
        let distance: f32 = ((point1.x - point0.x) * (point1.x - point0.x)
            + (point1.y - point0.y) * (point1.y - point0.y))
            .sqrt();
        distance
    }
}

impl RectangleDimensions for Rectangle {
    fn get_length(&self) -> i32 {
        todo!()
    }
    fn get_width(&self) -> i32 {
        todo!()
    }
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * (self.radius * self.radius)
    }

    pub fn perimter(&self) -> f32 {
        // println:  just to avoid the warnings for now.
        println!("{:?}", self.center);
        std::f32::consts::PI * self.radius * (2_f32)
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_from(&self, point: Point) -> f32 {
        self.distance_between_2_points([self.clone(), point])
    }
}

impl Triangle {
    pub fn new(points: [Point; 3]) -> Self {
        Self { points }
    }
    pub fn area(&self) -> f32 {
        todo!()
    }

    pub fn perimter(&self) -> f32 {
        // println:  just to avoid the warnings for now.
        println!("{:?}", self.points);
        todo!()
    }
}

impl Rectangle {
    pub fn new(points: [Point; 4]) -> Self {
        Self { points }
    }
    pub fn area(&self) -> f32 {
        todo!()
    }

    pub fn perimter(&self) -> f32 {
        // println:  just to avoid the warnings for now.
        println!("{:?}", self.points);
        todo!()
    }
}
