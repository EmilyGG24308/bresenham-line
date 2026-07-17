mod bmp;
mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::draw_line;

fn main() {
    let width = 400;
    let height = 400;
    let background = (10, 10, 20); // azul muy oscuro
    let mut fb = Framebuffer::new(width, height, background);

    // 1) Estrella con líneas saliendo del centro, cubriendo los 8 octantes
    //    con distintas pendientes en cada uno.
    let (cx, cy) = (200, 200);
    let radius = 150;
    fb.set_color((255, 200, 0));
    let n = 32;
    for i in 0..n {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        let x1 = cx + (radius as f64 * angle.cos()).round() as i32;
        let y1 = cy + (radius as f64 * angle.sin()).round() as i32;
        draw_line(&mut fb, cx, cy, x1, y1);
    }

    // 2) Casos explícitos por octante, para dejar constancia visual de que
    //    cada uno se traza correctamente (líneas cortas en cada esquina).
    let octant_cases: [(i32, i32, i32, i32, (u8, u8, u8)); 8] = [
        (20, 20, 70, 35, (255, 0, 0)),        // octante 1: dx>dy, x+ y+
        (20, 20, 35, 70, (0, 255, 0)),        // octante 2: dy>dx, x+ y+
        (380, 20, 330, 35, (0, 128, 255)),    // octante 3: dx>dy, x- y+
        (380, 20, 365, 70, (255, 0, 255)),    // octante 4: dy>dx, x- y+
        (380, 380, 330, 365, (255, 255, 0)),  // octante 5: dx>dy, x- y-
        (380, 380, 365, 330, (0, 255, 255)),  // octante 6: dy>dx, x- y-
        (20, 380, 70, 365, (255, 128, 0)),    // octante 7: dx>dy, x+ y-
        (20, 380, 35, 330, (128, 0, 255)),    // octante 8: dy>dx, x+ y-
    ];
    for (x0, y0, x1, y1, color) in octant_cases {
        fb.set_color(color);
        draw_line(&mut fb, x0, y0, x1, y1);
    }

    // 3) Líneas puramente horizontal y vertical (casos límite dx=0 / dy=0)
    fb.set_color((255, 255, 255));
    draw_line(&mut fb, 20, 200, 90, 200);
    draw_line(&mut fb, 200, 20, 200, 90);

    // Exportar a BMP
    bmp::write_bmp(&fb, "out.bmp").expect("no se pudo escribir out.bmp");
    println!("out.bmp generado ({}x{})", width, height);
}
