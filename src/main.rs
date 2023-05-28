mod vec3;

struct Image {
    width: i32,
    height: i32,
}

fn main() {

    // Image

    const IMAGE: Image = Image {
        width: 256,
        height: 256
    };
    
    // Render

    println!("P3
    {} {}
    255", IMAGE.width, IMAGE.height);

    for j in (0..IMAGE.height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE.width{
            let r= i as f32 / (IMAGE.width- 1) as f32;
            let g= j as f32 / (IMAGE.height- 1) as f32;
            let b= 0.25;

            // println!("{} {} {}", r, g, b);

            let ir = (255.999 as f32 * r) as i32;
            let ig = (255.999 as f32 * g) as i32 ;
            let ib = (255.999 as f32 * b) as i32 ;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("Done");
}
