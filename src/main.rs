use std::path::Path;
use std::cmp::min;
use image::{RgbImage};
use imageproc::drawing::{draw_convex_polygon};
use rtriangulate::{TriangulationPoint, triangulate, Triangle};

use edge_detection::Detection;
use rand::seq::SliceRandom;

const POINTS:u32 = 1200;
const RATE:f32 = 0.1;
const EDGE_THRESHOLD:f32 = 0.04;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn calculate_density(image:&Detection) -> Vec<Vec<f32>> {

    let mut matrix = Vec::new();

    let widht:u32 = image.width() as u32;
    let height:u32 = image.height() as u32;

    for i in 1..widht {
        let mut row = Vec::new();

        for j in 1..height {
            let mut sum = 0.0;

            for x in i-1..=i+1{
                for y in j-1..=j+1{
                    sum += image.interpolate(x as f32,y as f32).magnitude();
                }
            }

            row.push(sum/9.0)
        }
        matrix.push(row)
    }
        
    matrix.clone()
}

fn compute_color(p: &[TriangulationPoint<f32>])->(u32, u32){
    let mut x = 0.0;
    let mut y = 0.0;

    for k in p{
        x += k.x;
        y += k.y;
    }

    x = x/3.0;
    y = y/3.0;

    return (x as u32, y as u32)
}

fn compute_image(points:&Vec<TriangulationPoint<f32>>, 
                    triangles:&Vec<Triangle>, 
                    image: &RgbImage,
                    original: &RgbImage) -> RgbImage{
    
    let mut cl = image.clone();
    let mut i = 1;
    let total = triangles.len();
    for triangle in triangles{
        println!("{} of {}", i, total);
        let p1 = Point::new(points[triangle.0].x as i32, points[triangle.0].y as i32);
        let p2 = Point::new(points[triangle.1].x as i32, points[triangle.1].y as i32);
        let p3 = Point::new(points[triangle.2].x as i32, points[triangle.2].y as i32);

        let p = vec![points[triangle.0], points[triangle.1],points[triangle.2]];

        let baricenter = compute_color(p.as_slice());

        let color = original.get_pixel(baricenter.0,baricenter.1);

        cl = draw_convex_polygon(&cl, &[imageproc::drawing::Point::new(p1.x, p1.y), imageproc::drawing::Point::new(p2.x, p2.y), imageproc::drawing::Point::new(p3.x, p3.y)], *color);
        i+=1;
    }

    cl

}

fn main() {


    let path = Path::new("output.png");
    let mut rng = &mut rand::thread_rng();
    
    let original = image::open("resources/test2.jpeg")
        .expect("failed to read image");
    
    let source_image = original.to_luma();
    let detection = edge_detection::canny(
        source_image,
        1.2,  // sigma
        0.2,  // strong threshold
        0.01, // weak threshold
    );

    //image.save(path).unwrap();
    let matrix = calculate_density(&detection);

    let mut candidates = Vec::new();

    // select the pixels with desired intensity
    for x in 0..matrix.len(){
        for y in 0..matrix[0].len(){
            if matrix[x][y] > EDGE_THRESHOLD{
                candidates.push(TriangulationPoint::new(x as f32,y as f32));
            }
        }
    }

    let size = (&detection.width() * &detection.height()) as f32;
    let p = min(POINTS, (RATE*size) as u32);

    let points:Vec<TriangulationPoint<f32>> = candidates.choose_multiple(&mut rng, p as usize).cloned().collect();

    
    let triangles = triangulate(&points).unwrap();

    //println!("{} {}", candidates.len(), points.len())
    //println!("{:?}", triangles);
    //println!("{}", triangles[0].0)

    let mut result = RgbImage::new(detection.width() as u32, detection.height() as u32);
    let image_result = compute_image(&points, &triangles, &result, &original.to_rgb());

    image_result.save(path);
}
