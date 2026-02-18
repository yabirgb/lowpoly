use std::path::Path;
use std::cmp::min;
use std::env;
use image::{Rgb, RgbImage};
use rtriangulate::{TriangulationPoint, triangulate, Triangle};

use edge_detection::Detection;
use rand::seq::SliceRandom;

const POINTS:u32 = 1200;
const RATE:f32 = 0.1;
const EDGE_THRESHOLD:f32 = 0.04;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn collect_candidates(image: &Detection) -> Vec<TriangulationPoint<f32>> {
    let mut candidates = Vec::new();

    let width = image.width() as u32;
    let height = image.height() as u32;
    if width < 2 || height < 2 {
        return candidates;
    }

    for i in 1..width {
        for j in 1..height {
            let mut sum = 0.0;

            for x in i - 1..=i + 1 {
                for y in j - 1..=j + 1 {
                    sum += image.interpolate(x as f32, y as f32).magnitude();
                }
            }

            if (sum / 9.0) > EDGE_THRESHOLD {
                candidates.push(TriangulationPoint::new(i as f32, j as f32));
            }
        }
    }

    candidates
}

fn compute_image(points:&Vec<TriangulationPoint<f32>>, 
                    triangles:&Vec<Triangle>, 
                    image: &RgbImage,
                    original: &RgbImage) -> RgbImage{
    
    let mut cl = image.clone();
    for triangle in triangles{
        let p1 = Point::new(points[triangle.0].x as i32, points[triangle.0].y as i32);
        let p2 = Point::new(points[triangle.1].x as i32, points[triangle.1].y as i32);
        let p3 = Point::new(points[triangle.2].x as i32, points[triangle.2].y as i32);

        let x = (points[triangle.0].x + points[triangle.1].x + points[triangle.2].x) / 3.0;
        let y = (points[triangle.0].y + points[triangle.1].y + points[triangle.2].y) / 3.0;
        let baricenter = (x as u32, y as u32);

        let x = min(baricenter.0, original.width().saturating_sub(1));
        let y = min(baricenter.1, original.height().saturating_sub(1));
        let color = original.get_pixel(x, y);

        draw_filled_triangle_mut(&mut cl, p1, p2, p3, *color);
    }

    cl

}

fn edge(a: Point, b: Point, p: Point) -> i64 {
    (p.x - a.x) as i64 * (b.y - a.y) as i64 - (p.y - a.y) as i64 * (b.x - a.x) as i64
}

fn draw_filled_triangle_mut(image: &mut RgbImage, p1: Point, p2: Point, p3: Point, color: Rgb<u8>) {
    let width = image.width() as i32;
    let height = image.height() as i32;
    if width <= 0 || height <= 0 {
        return;
    }

    let min_x = p1.x.min(p2.x).min(p3.x).max(0);
    let max_x = p1.x.max(p2.x).max(p3.x).min(width - 1);
    let min_y = p1.y.min(p2.y).min(p3.y).max(0);
    let max_y = p1.y.max(p2.y).max(p3.y).min(height - 1);

    if min_x > max_x || min_y > max_y {
        return;
    }

    let area = edge(p1, p2, p3);
    if area == 0 {
        return;
    }

    let p1x = p1.x as i64;
    let p1y = p1.y as i64;
    let p2x = p2.x as i64;
    let p2y = p2.y as i64;
    let p3x = p3.x as i64;
    let p3y = p3.y as i64;

    let w0_dx = p3y - p2y;
    let w1_dx = p1y - p3y;
    let w2_dx = p2y - p1y;

    let w0_dy = -(p3x - p2x);
    let w1_dy = -(p1x - p3x);
    let w2_dy = -(p2x - p1x);

    let start_x = min_x as i64;
    let mut w0_row = edge(p2, p3, Point::new(min_x, min_y));
    let mut w1_row = edge(p3, p1, Point::new(min_x, min_y));
    let mut w2_row = edge(p1, p2, Point::new(min_x, min_y));

    for y in min_y..=max_y {
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;
        let mut x = start_x;

        while x <= max_x as i64 {
            let inside = if area > 0 {
                w0 >= 0 && w1 >= 0 && w2 >= 0
            } else {
                w0 <= 0 && w1 <= 0 && w2 <= 0
            };

            if inside {
                image.put_pixel(x as u32, y as u32, color);
            }

            w0 += w0_dx;
            w1 += w1_dx;
            w2 += w2_dx;
            x += 1;
        }

        w0_row += w0_dy;
        w1_row += w1_dy;
        w2_row += w2_dy;
    }
}

fn add_border_points(points: &mut Vec<TriangulationPoint<f32>>, width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }

    let max_x = width.saturating_sub(1);
    let max_y = height.saturating_sub(1);

    points.push(TriangulationPoint::new(0.0, 0.0));
    points.push(TriangulationPoint::new(max_x as f32, 0.0));
    points.push(TriangulationPoint::new(0.0, max_y as f32));
    points.push(TriangulationPoint::new(max_x as f32, max_y as f32));

    let segments = 12;
    for i in 1..segments {
        let t = i as f32 / segments as f32;
        let x = (t * max_x as f32).round();
        let y = (t * max_y as f32).round();

        points.push(TriangulationPoint::new(x, 0.0));
        points.push(TriangulationPoint::new(x, max_y as f32));
        points.push(TriangulationPoint::new(0.0, y));
        points.push(TriangulationPoint::new(max_x as f32, y));
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_image_path> <output_image_path>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let mut rng = &mut rand::thread_rng();
    
    let original = image::open(Path::new(input_path))
        .expect("failed to read image");
    
    let source_image = original.to_luma();
    let detection = edge_detection::canny(
        source_image,
        1.2,  // sigma
        0.2,  // strong threshold
        0.01, // weak threshold
    );

    let candidates = collect_candidates(&detection);

    let size = (&detection.width() * &detection.height()) as f32;
    let p = min(POINTS, (RATE*size) as u32);

    let mut points:Vec<TriangulationPoint<f32>> = candidates.choose_multiple(&mut rng, p as usize).cloned().collect();
    add_border_points(&mut points, detection.width() as u32, detection.height() as u32);


    let triangles = triangulate(&points).unwrap();

    //println!("{} {}", candidates.len(), points.len())
    //println!("{:?}", triangles);
    //println!("{}", triangles[0].0)

    let result = RgbImage::new(detection.width() as u32, detection.height() as u32);
    let image_result = compute_image(&points, &triangles, &result, &original.to_rgb());

    image_result
        .save(Path::new(output_path))
        .expect("failed to save output image");
}
