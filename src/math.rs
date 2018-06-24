use num_traits::Float;
use num_traits::Num;
use num_traits::AsPrimitive;
use cgmath::{
    Vector2,
    Vector3,
    Point2,
    BaseNum,
    BaseFloat,
    Zero,
    InnerSpace,
};
use std::ops::Add;
use std;
use sdl2::pixels::Color;
use std::cmp::max;
use std::cmp::min;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RectBounds<T> {
    pub top: T,
    pub bottom: T,
    pub right: T,
    pub left: T,
}

impl <T: 'static> RectBounds<T> {
    pub fn bounds_of(points: &Vec<Point2<T>>) -> Option<RectBounds<T>>
        where T: Num + PartialOrd + Copy {
        let bottom = points.iter()
            .min_by(|p1, p2| p1.y.partial_cmp(&p2.y).unwrap())
            .map(|p| p.y);
        let top = points.iter()
            .max_by(|p1, p2| p1.y.partial_cmp(&p2.y).unwrap())
            .map(|p| p.y);
        let left = points.iter()
            .min_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap())
            .map(|p| p.x);
        let right = points.iter()
            .max_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap())
            .map(|p| p.x);
        return match (bottom, top, left, right) {
            (Some(b), Some(t), Some(l), Some(r)) => Some(RectBounds {
                top: t,
                bottom: b,
                left: l,
                right: r
            }),
            _ => None,
        }
    }

    pub fn overlap(&self, bounds: RectBounds<T>) -> RectBounds<T> where T: BaseNum + Ord {
        let mut overlap = *self;
        if overlap.left < bounds.left {
            overlap.left = min(self.right, bounds.left);
        }
        if overlap.right > bounds.right {
            overlap.right = max(self.left, bounds.right);
        }
        if overlap.bottom < bounds.bottom {
            overlap.bottom = min(self.top, bounds.bottom);
        }
        if overlap.top > bounds.top {
            overlap.top = max(self.bottom, bounds.top);
        }
        overlap
    }

    pub fn bounds_of_triangle(triangle: Triangle<T>) -> RectBounds<T> where T: BaseNum {
        return Self::bounds_of(&vec![triangle.p0, triangle.p1, triangle.p2]).unwrap();
    }

    pub fn from<U>(bounds: RectBounds<U>) -> Self where U: AsPrimitive<T>, T: Copy {
        return RectBounds{
            top: bounds.top.as_(),
            bottom: bounds.bottom.as_(),
            left: bounds.left.as_(),
            right: bounds.right.as_(),
        };
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle<T> {
    pub p0: Point2<T>,
    pub p1: Point2<T>,
    pub p2: Point2<T>,
}

impl <T> Triangle<T> {
    pub fn from_slice(points: &[Point2<T>]) -> Option<Triangle<T>> where T: Copy {
        if points.len() != 3 {
            return None;
        } else {
            return Some(Triangle{
                p0: points[0],
                p1: points[1],
                p2: points[2],
            });
        }
    }

    pub fn point_is_inside(&self, point: Point2<T>) -> bool where T: Float + Copy {
        // Note: we assume that the triangle's points are ordered counter-clockwise, which is true
        // because we use a right-hand coordinate system.
        return on_right(point, self.p0, self.p2)
            && on_right(point, self.p2, self.p1)
            && on_right(point, self.p1, self.p0);
    }

    pub fn area(&self) -> T where T: BaseFloat {
        return (self.p1.x - self.p0.x) * (self.p2.y - self.p0.y)
            - (self.p1.y - self.p0.y) * (self.p2.x - self.p0.x);
    }

    pub fn barycentric_coordinates(&self, point: Point2<T>) -> (T, T, T) where T: BaseFloat {
        let area = self.area();
        let p0_triangle = Triangle{
            p0: point,
            p1: self.p1,
            p2: self.p2,
        };
        let p0_coordinate = p0_triangle.area() / area;
        let p1_triangle = Triangle{
            p0: point,
            p1: self.p2,
            p2: self.p0,
        };
        let p1_coordinate = p1_triangle.area() / area;
        let p2_coordinate = T::one() - p1_coordinate - p0_coordinate;
        return (p0_coordinate, p1_coordinate, p2_coordinate);
    }
}

pub fn on_right<T: Float + Copy>(point: Point2<T>, line_p1: Point2<T>, line_p2: Point2<T>) -> bool {
    // on_right OR on, really
    return (point.x - line_p1.x) * (line_p2.y - line_p1.y) -
        (point.y - line_p1.y) * (line_p2.x - line_p1.x) >= T::zero();
}

pub fn mix_colors(colors: &Vec<Color>, weights: &Vec<f32>) -> Color {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for (i, color) in colors.iter().enumerate() {
        let weight = weights.get(i).unwrap_or(&0.0);
        r += (weight * (color.r as f32)) as u8;
        g += (weight * (color.g as f32)) as u8;
        b += (weight * (color.b as f32)) as u8;
    }
    return Color{a: 255, r, g, b};
}

/*
pub fn barycentric_coordinates<T: Float + Copy>(point: Point2<T>, triangle: Triangle<T>) -> (T, T, T) {

}
*/



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_of() {
        assert_eq!(
            RectBounds::<u32>::bounds_of(&vec![Point2{x: 0, y: 0}]),
            Some(RectBounds{left: 0, right: 0, top: 0, bottom: 0}));
        assert_eq!(
            RectBounds::<u32>::bounds_of(&vec![
                Point2{x: 0, y: 0},
                Point2{x: 1, y: 1},
            ]),
            Some(RectBounds{left: 0, right: 1, top: 1, bottom: 0}));
        assert_eq!(
            RectBounds::<u32>::bounds_of(&vec![
                Point2{x: 0, y: 0},
                Point2{x: 1, y: 1},
                Point2{x: 0, y: 3},
            ]),
            Some(RectBounds{left: 0, right: 1, top: 3, bottom: 0}));
    }

    #[test]
    fn test_on_right() {
        assert!(on_right(
            Point2{x: 0.0, y: 0.0},
            Point2{x: 0.0, y: 5.0},
            Point2{x: 1.0, y: 0.0}));
        assert!(!on_right(
            Point2{x: 0.0, y: 0.0},
            Point2{x: 0.0, y: 5.0},
            Point2{x: -1.0, y: 0.0}));
    }

    #[test]
    fn test_triangle_area() {
        assert_eq!(Triangle{
            p0: Point2{x: 0.0, y: 0.0},
            p1: Point2{x: 0.0, y: 1.0},
            p2: Point2{x: 1.0, y: 0.0},
        }.area(), 0.5);
    }

    #[test]
    fn test_triangle_bary() {
        assert_eq!(Triangle{
            p0: Point2{x: 0.0, y: 0.0},
            p1: Point2{x: 0.0, y: 1.0},
            p2: Point2{x: 1.0, y: 0.0},
        }.barycentric_coordinates(Point2{x: 0.0, y: 0.0}), (1.0, 0.0, 0.0));
        assert_eq!(Triangle{
            p0: Point2{x: 0.0, y: 0.0},
            p1: Point2{x: 0.0, y: 1.0},
            p2: Point2{x: 1.0, y: 0.0},
        }.barycentric_coordinates(Point2{x: 0.0, y: 1.0}), (0.0, 1.0, 0.0));
        assert_eq!(Triangle{
            p0: Point2{x: 0.0, y: 0.0},
            p1: Point2{x: 0.0, y: 1.0},
            p2: Point2{x: 1.0, y: 0.0},
        }.barycentric_coordinates(Point2{x: 1.0, y: 0.0}), (0.0, 0.0, 1.0));
        assert_eq!(Triangle{
            p0: Point2{x: 0.0, y: 0.0},
            p1: Point2{x: 0.0, y: 1.0},
            p2: Point2{x: 1.0, y: 0.0},
        }.barycentric_coordinates(Point2{x: 0.5, y: 0.0}), (0.5, 0.0, 0.5));
    }
}

