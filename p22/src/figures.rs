#[derive(Debug, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
pub struct Circle {
    _center: Point,
    radius: f32,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    points: [Point; 3],
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug, Clone)]
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

fn distance_between_2_points(points: [Point; 2]) -> f32 {
    let point0: Point = points[0].clone();
    let point1: Point = points[1].clone();
    let distance: f32 = ((point1.x - point0.x) * (point1.x - point0.x)
        + (point1.y - point0.y) * (point1.y - point0.y))
        .sqrt();
    distance
}

pub fn circle_perimeter_and_area(figure: Circle) -> (f32, f32) {
    let perimeter: f32 = std::f32::consts::PI * (figure.radius * 2.0);
    let area: f32 = std::f32::consts::PI * figure.radius * figure.radius;
    (perimeter, area)
}

pub fn triangle_perimeter_and_area(figure: Triangle) -> (f32, f32) {
    let points = figure.points;
    let point1 = &points[0];
    let point2 = &points[1];
    let point3 = &points[2];

    let side1 = distance_between_2_points([point1.clone(), point2.clone()]);
    let side2 = distance_between_2_points([point1.clone(), point3.clone()]);
    let side3 = distance_between_2_points([point2.clone(), point3.clone()]);
    let perimeter = side1 + side2 + side3;
    let semi_per = perimeter / 2_f32;
    let area: f32 =
        (semi_per * (semi_per - side1) * (semi_per - side2) * (semi_per - side3)).sqrt();
    (perimeter, area)
}

pub fn point_perimeter_and_area(_point: Point) -> (f32, f32) {
    (0_f32, 0_f32)
}

pub fn rectangle_perimeter_and_area(figure: Rectangle) -> (f32, f32) {
    let top_left = figure.top_left;
    let bottom_right = figure.bottom_right;
    let height: f32 = top_left.y - bottom_right.y;
    let width: f32 = bottom_right.x - top_left.x;
    let perimeter = (height + width) * 2_f32;
    let area: f32 = height * width;
    (perimeter, area)
}

pub fn shape_perimeter_and_area(shape: Shape) -> (f32, f32) {
    match shape {
        Shape::Circle(circle) => circle_perimeter_and_area(circle),
        Shape::Point(point) => point_perimeter_and_area(point),
        Shape::Triangle(triangle) => triangle_perimeter_and_area(triangle),
        Shape::Rectangle(rectangle) => rectangle_perimeter_and_area(rectangle),
    }
}

#[cfg(test)]

mod tests {
    use crate::figures::{
        circle_perimeter_and_area, point_perimeter_and_area, rectangle_perimeter_and_area,
        shape_perimeter_and_area, triangle_perimeter_and_area, Circle, Rectangle, Shape,
    };

    use super::{Point, Triangle};
    #[test]
    fn test_triangle() {
        let point1: Point = Point { x: 2.0, y: 1.0 };
        let point2: Point = Point { x: 2.0, y: 4.0 };
        let point3: Point = Point { x: 6.0, y: 1.0 };
        let triangle: Triangle = Triangle {
            points: [point1, point2, point3],
        };
        assert_eq!((12.0, 6.0), triangle_perimeter_and_area(triangle));
    }

    #[test]
    fn test_rectangle() {
        let top_left: Point = Point { x: 2.0, y: 4.0 };
        let bottom_right: Point = Point { x: 6.0, y: 1.0 };

        let rectangle: Rectangle = Rectangle {
            top_left: top_left,
            bottom_right: bottom_right,
        };
        assert_eq!((14.0, 12.0), rectangle_perimeter_and_area(rectangle));
    }

    #[test]
    fn test_circle() {
        let center: Point = Point { x: 2.0, y: 4.0 };
        let radius: f32 = 3_f32;

        let circle: Circle = Circle {
            _center: center,
            radius: radius,
        };
        assert_eq!(
            (std::f32::consts::PI * 6.0, std::f32::consts::PI * 9.0),
            circle_perimeter_and_area(circle)
        );
    }

    #[test]
    fn test_point() {
        let point: Point = Point { x: 2.0, y: 4.0 };
        assert_eq!((0.0, 0.0), point_perimeter_and_area(point));
    }

    #[test]
    fn test_shape() {
        let point1: Point = Point { x: 2.0, y: 1.0 };
        let point2: Point = Point { x: 2.0, y: 4.0 };
        let point3: Point = Point { x: 6.0, y: 1.0 };
        let triangle: Triangle = Triangle {
            points: [point1.clone(), point2.clone(), point3.clone()],
        };
        let shape1: Shape = Shape::Triangle(triangle);

        assert_eq!((12.0, 6.0), shape_perimeter_and_area(shape1));

        let rectangle: Rectangle = Rectangle {
            top_left: point2.clone(),
            bottom_right: point3.clone(),
        };
        let shape2: Shape = Shape::Rectangle(rectangle);
        assert_eq!((14.0, 12.0), shape_perimeter_and_area(shape2));

        let radius: f32 = 3_f32;
        let circle: Circle = Circle {
            _center: point2.clone(),
            radius: radius,
        };
        let shape3: Shape = Shape::Circle(circle);
        assert_eq!(
            (std::f32::consts::PI * 6.0, std::f32::consts::PI * 9.0),
            shape_perimeter_and_area(shape3)
        );
        let shape4: Shape = Shape::Point(point1.clone());
        assert_eq!((0.0, 0.0), shape_perimeter_and_area(shape4));
    }
}
