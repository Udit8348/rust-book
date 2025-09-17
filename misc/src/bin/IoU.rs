fn main() {
    println!("\n🦀 >> Intersection over Union << 🦀\n");

    // print the values of a vector
    let rect1: Vec<i32> = vec![10,15,16,8];
    let rect2: Vec<i32> = vec![9,14,13,6];
    
    println!("Starting with the two following rectangles {:?} and {:?} lets compute IoU", rect1, rect2);

    let left = std::cmp::max(rect1[0], rect2[0]);
    let right = std::cmp::min(rect1[2], rect2[2]);
    let top = std::cmp::min(rect1[1], rect2[1]);
    let bottom = std::cmp::max(rect1[3], rect2[3]);

    let width = right - left;
    let height = top - bottom;
    let intersection_area= width * height;

    let area1 = (rect1[2] - rect1[0]) * (rect1[1] - rect1[3]);
    let area2 = (rect2[2] - rect2[0]) * (rect2[1] - rect2[3]);
    let union = area1 + area2 - intersection_area;

    println!("The IoU: {}", intersection_area as f64 / union as f64); // compute in floating point
}