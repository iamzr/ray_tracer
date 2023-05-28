fn main() {
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    
    // Render

    println!("P3
    {} {}
    255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r= i as f64 / (IMAGE_WIDTH- 1) as f64;
            let g= j as f64 / (IMAGE_HEIGHT- 1) as f64;
            let b= 0.25;

            // println!("{} {} {}", r, g, b);

            let ir = (255.999 as f64 * r) as i64;
            let ig = (255.999 as f64 * g) as i64 ;
            let ib = (255.999 as f64 * b) as i64 ;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
