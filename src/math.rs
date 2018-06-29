use num_traits::Float;
use num_traits::Num;
use num_traits::AsPrimitive;
use cgmath::{
    BaseFloat,
    BaseNum,
    Point2,
    Vector2,
    Vector3,
    Point3,
};
use std;
use sdl2::pixels::Color;
use std::cmp::max;
use std::cmp::min;
use collision::Ray;
use collision::Line;
use collision::Continuous;
use collision::Plane;

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

    pub fn overlap(&self, bounds: RectBounds<T>) -> Option<RectBounds<T>> where T: BaseNum + Ord {
        if self.right < bounds.left
            || self.left > bounds.right
            || self.top < bounds.bottom
            || self.bottom > bounds.top {
            return None;
        }
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
        Some(overlap)
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

    pub fn bounds_at_height(&self, y: T) -> Option<(T, T)> where T: BaseFloat {
        let l01 = Line::new(self.p0, self.p1);
        let l02 = Line::new(self.p0, self.p2);
        let l12 = Line::new(self.p1, self.p2);
        let r1 = Ray::new(
            Point2{x: T::zero(), y},
            Vector2{x: T::one(), y: T::zero()}
        );
        let r2 = Ray::new(
            Point2{x: T::zero(), y},
            Vector2{x: -T::one(), y: T::zero()}
        );
        let points = [
            r1.intersection(&l01),
            r1.intersection(&l02),
            r1.intersection(&l12),
            r2.intersection(&l01),
            r2.intersection(&l02),
            r2.intersection(&l12),
        ];
        let optional_min = points.iter()
            .filter_map(|optional_point| *optional_point)
            .map(|point| point.x)
            .min_by(|x, y| if x > y { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less });
        let optional_max = points.iter()
            .filter_map(|optional_point| *optional_point)
            .map(|point| point.x)
            .max_by(|x, y| if x > y { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less });
        match (optional_min, optional_max) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
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
        return (((self.p1.x - self.p0.x) * (self.p2.y - self.p0.y)
            - (self.p1.y - self.p0.y) * (self.p2.x - self.p0.x)) / (T::one() + T::one())).abs();
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

// vertices are kept if they're on the side of the plane indicated by the normal vector
pub fn clip<T>(points: &Vec<Point3<T>>, plane: &Plane<T>) -> Vec<Point3<T>> where T: BaseFloat {
    if points.len() == 0 {
        return vec![];
    }
    let mut new_points: Vec<Point3<T>> = vec![];
    let mut above = point_above_plane(&points[points.len() - 1], plane);
    for (i, point) in points.iter().enumerate() {
        let prev_point = match i {
            0 => points[points.len() - 1],
            _ => points[i - 1],
        };
        if point_above_plane(point, plane) {
            if above == false {
                // crossed back over!
                let ray = Ray::new(prev_point, *point - prev_point);
                let intersection = plane.intersection(&ray).unwrap();
                new_points.push(intersection);
                new_points.push(*point);
            } else {
                // we were above, and still are!
                new_points.push(*point);
            }
            above = true;
        } else {
            if above == true {
                // crossed below! add our intersection point and continue
                let ray = Ray::new(prev_point, *point - prev_point);
                let intersection = plane.intersection(&ray).unwrap();
                new_points.push(intersection);
            } else {
                // still below! do nothing at all!
            }
            above = false;
        }
    }
    new_points
}

// lots of optimizations to be made here
pub fn clip_in_box<T>(points: &Vec<Point3<T>>) -> Vec<Point3<T>> where T: BaseFloat {
    let mut clipped_points = clip(
        points,
        &Plane::new(Vector3::unit_x(), T::one()),
    );
    clipped_points = clip(
        &clipped_points,
        &Plane::new(-Vector3::unit_x(), T::one()),
    );
    clipped_points = clip(
        &clipped_points,
        &Plane::new(Vector3::unit_y(), T::one()),
    );
    clipped_points = clip(
        &clipped_points,
        &Plane::new(-Vector3::unit_y(), T::one()),
    );
    clipped_points = clip(
        &clipped_points,
        &Plane::new(Vector3::unit_z(), T::one()),
    );
    clipped_points = clip(
        &clipped_points,
        &Plane::new(-Vector3::unit_z(), T::one()),
    );
    return clipped_points;
}

pub fn convex_triangulation<T>(points: &Vec<Point3<T>>) -> Vec<(Point3<T>, Point3<T>, Point3<T>)>
    where T: BaseFloat {
    if points.len() < 3 {
        return vec![];
    }
    let mut tris = vec![];
    for i in 2..points.len() {
        tris.push((points[0], points[i - 1], points[i]));
    }
    return tris;
}

// TODO: bug in collision library
pub fn point_above_plane<T>(point: &Point3<T>, plane: &Plane<T>) -> bool where T: BaseFloat {
    point.x * plane.n.x + point.y * plane.n.y + point.z * plane.n.z + plane.d > T::zero()
}

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

    #[test]
    fn test_clip_with_no_clipping() {
        let triangle = vec![
            Point3{x: -1.0, y: 0.0, z: -1.0},
            Point3{x: 1.0, y: 0.0, z: 1.0},
            Point3{x: -1.0, y: 0.0, z: 1.0},
        ];
        let clipped_triangle = vec![
            Point3{x: -1.0, y: 0.0, z: -1.0},
            Point3{x: 1.0, y: 0.0, z: 1.0},
            Point3{x: -1.0, y: 0.0, z: 1.0},
        ];
        let plane = Plane::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1.0);
        assert_eq!(clip(&triangle, &plane), clipped_triangle);
    }

    #[test]
    fn test_clip_with_all_clipping() {
        let triangle = vec![
            Point3{x: -1.0, y: 0.0, z: -1.0},
            Point3{x: 1.0, y: 0.0, z: 1.0},
            Point3{x: -1.0, y: 0.0, z: 1.0},
        ];
        let clipped_triangle = vec![];
        let plane = Plane::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, -2.0);
        assert_eq!(clip(&triangle, &plane), clipped_triangle);
    }

    #[test]
    fn test_clip() {
        let triangle = vec![
            Point3{x: -1.0, y: 0.0, z: -1.0},
            Point3{x: 1.0, y: 0.0, z: 1.0},
            Point3{x: -1.0, y: 0.0, z: 1.0},
        ];
        let clipped_triangle = vec![
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 0.0, z: 1.0},
            Point3{x: 0.0, y: 0.0, z: 1.0},
        ];
        let plane = Plane::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 0.0);
        assert_eq!(clip(&triangle, &plane), clipped_triangle);
    }

    #[test]
    fn test_clip_again() {
        let triangle = vec![
            Point3{x: -2.0, y: 0.0, z: -2.0},
            Point3{x: 1.0, y: 0.0, z: -2.0},
            Point3{x: 1.0, y: 0.0, z: -1.0},
            Point3{x: -1.0, y: 0.0, z: -1.0},
            Point3{x: -1.0, y: 0.0, z: 1.0},
            Point3{x: 1.0, y: 0.0, z: 1.0},
            Point3{x: 1.0, y: 0.0, z: 2.0},
            Point3{x: -2.0, y: 0.0, z: 2.0},
        ];
        let clipped_triangle = vec![
            Point3{x: -2.0, y: 0.0, z: -2.0},
            Point3{x: 0.0, y: 0.0, z: -2.0},
            Point3{x: 0.0, y: 0.0, z: -1.0},
            Point3{x: -1.0, y: 0.0, z: -1.0},
            Point3{x: -1.0, y: 0.0, z: 1.0},
            Point3{x: 0.0, y: 0.0, z: 1.0},
            Point3{x: 0.0, y: 0.0, z: 2.0},
            Point3{x: -2.0, y: 0.0, z: 2.0},
        ];
        let plane = Plane::new(Vector3{x: -1.0, y: 0.0, z: 0.0}, 0.0);
        assert_eq!(clip(&triangle, &plane), clipped_triangle);
    }

    #[test]
    fn test_clip_in_box() {
        assert_eq!(clip_in_box(&vec![
            Point3{x: 0.5, y: 0.5, z: 0.0},
            Point3{x: 0.5, y: -0.5, z: 0.0},
            Point3{x: -0.5, y: 0.0, z: 0.0},
        ]), vec![
            Point3{x: 0.5, y: 0.5, z: 0.0},
            Point3{x: 0.5, y: -0.5, z: 0.0},
            Point3{x: -0.5, y: 0.0, z: 0.0},
        ]);
        assert_eq!(clip_in_box(&vec![
            Point3{x: 3.0, y: 3.0, z: 0.0},
            Point3{x: 3.0, y: -3.0, z: 0.0},
            Point3{x: -3.0, y: 0.0, z: 0.0},
        ]), vec![
            Point3{x: -1.0, y: -1.0, z: 0.0},
            Point3{x: -1.0, y: 1.0, z: 0.0},
            Point3{x: 1.0, y: 1.0, z: 0.0},
            Point3{x: 1.0, y: -1.0, z: 0.0},
        ]);
    }

    #[test]
    fn test_triangulation() {
        assert_eq!(convex_triangulation(&vec![
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 0.0, z: 0.0},
            Point3{x: 0.0, y: 1.0, z: 0.0},
        ]), vec![(
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 0.0, z: 0.0},
            Point3{x: 0.0, y: 1.0, z: 0.0},
        )]);
        assert_eq!(convex_triangulation(&vec![
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 1.0, z: 0.0},
            Point3{x: 0.0, y: 1.0, z: 0.0},
        ]), vec![(
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 1.0, z: 0.0},
        ), (
            Point3{x: 0.0, y: 0.0, z: 0.0},
            Point3{x: 1.0, y: 1.0, z: 0.0},
            Point3{x: 0.0, y: 1.0, z: 0.0},
        )]);
    }

    #[test]
    fn repro() {
        let plane = Plane::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 5.0);
        let ray = Ray::new(
            Point3{x: -10.0, y: 0.0, z: 0.0},
            Vector3{x: 1.0, y: 0.0, z: 0.0},
        );
        println!("Intersection: {:?}", plane.intersection(&ray));
    }
}

