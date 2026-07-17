use crate::framebuffer::Framebuffer;

/// Dibuja una línea desde (x0, y0) hasta (x1, y1) sobre el framebuffer
/// usando el algoritmo de Bresenham, con aritmética exclusivamente entera.
///
/// Maneja los 8 octantes (cualquier combinación de dirección en x/y y
/// pendientes mayores o menores a 1) mediante:
///   - dx, dy con signo (para saber la dirección de avance: sx, sy)
///   - dy negativo (-|dy|) como término del error, técnica estándar que
///     evita tener que distinguir "línea más horizontal" de "línea más
///     vertical" con dos bucles separados.
pub fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32) {
    let mut x = x0;
    let mut y = y0;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    // Dirección del paso en cada eje: -1, 0 o 1
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };

    // Término de error acumulado (todo entero)
    let mut err = dx + dy;

    loop {
        fb.point(x, y);

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        // Avanza en x si el error lo permite
        if e2 >= dy {
            err += dy;
            x += sx;
        }

        // Avanza en y si el error lo permite
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drawn_points(fb: &Framebuffer, bg: (u8, u8, u8)) -> Vec<(i32, i32)> {
        let mut pts = Vec::new();
        for y in 0..fb.height {
            for x in 0..fb.width {
                let idx = (y * fb.width + x) * 3;
                let px = (fb.data[idx], fb.data[idx + 1], fb.data[idx + 2]);
                if px != bg {
                    pts.push((x as i32, y as i32));
                }
            }
        }
        pts
    }

    #[test]
    fn test_horizontal_line() {
        let bg = (0, 0, 0);
        let mut fb = Framebuffer::new(10, 10, bg);
        fb.set_color((255, 255, 255));
        draw_line(&mut fb, 1, 5, 8, 5);
        let pts = drawn_points(&fb, bg);
        assert_eq!(pts.len(), 8);
        assert!(pts.iter().all(|&(_, y)| y == 5));
    }

    #[test]
    fn test_vertical_line() {
        let bg = (0, 0, 0);
        let mut fb = Framebuffer::new(10, 10, bg);
        fb.set_color((255, 255, 255));
        draw_line(&mut fb, 3, 1, 3, 8);
        let pts = drawn_points(&fb, bg);
        assert_eq!(pts.len(), 8);
        assert!(pts.iter().all(|&(x, _)| x == 3));
    }

    #[test]
    fn test_diagonal_45_degrees() {
        let bg = (0, 0, 0);
        let mut fb = Framebuffer::new(10, 10, bg);
        fb.set_color((255, 255, 255));
        draw_line(&mut fb, 0, 0, 5, 5);
        let pts = drawn_points(&fb, bg);
        assert_eq!(pts.len(), 6);
        for (x, y) in pts {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn test_all_octants_endpoints() {
        // Verifica que en cada octante la línea inicia y termina exactamente
        // en los puntos solicitados.
        let bg = (0, 0, 0);
        let cases = [
            (20, 20, 25, 22), // octante 1 (dx>dy>0)
            (20, 20, 22, 25), // octante 2 (dy>dx>0)
            (20, 20, 18, 25), // octante 3
            (20, 20, 15, 22), // octante 4
            (20, 20, 15, 18), // octante 5
            (20, 20, 18, 15), // octante 6
            (20, 20, 22, 15), // octante 7
            (20, 20, 25, 18), // octante 8
        ];

        for (x0, y0, x1, y1) in cases {
            let mut fb = Framebuffer::new(40, 40, bg);
            fb.set_color((255, 255, 255));
            draw_line(&mut fb, x0, y0, x1, y1);

            let idx0 = (y0 as usize * fb.width + x0 as usize) * 3;
            let idx1 = (y1 as usize * fb.width + x1 as usize) * 3;
            assert_ne!((fb.data[idx0], fb.data[idx0 + 1], fb.data[idx0 + 2]), bg);
            assert_ne!((fb.data[idx1], fb.data[idx1 + 1], fb.data[idx1 + 2]), bg);
        }
    }
}
