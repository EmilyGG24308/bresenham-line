/// Un framebuffer simple en memoria: un arreglo de píxeles RGB (u8, u8, u8)
/// almacenado en un vector plano (row-major), con ancho y alto fijos.
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    /// Datos en formato RGB, 3 bytes por píxel, fila por fila desde arriba.
    pub data: Vec<u8>,
    /// Color actual usado para dibujar (se puede cambiar con set_color).
    color: (u8, u8, u8),
}

impl Framebuffer {
    /// Crea un framebuffer del tamaño dado, relleno con el color de fondo.
    pub fn new(width: usize, height: usize, background: (u8, u8, u8)) -> Self {
        let mut fb = Framebuffer {
            width,
            height,
            data: vec![0u8; width * height * 3],
            color: (255, 255, 255),
        };
        fb.clear(background);
        fb
    }

    /// Rellena todo el framebuffer con un color sólido.
    pub fn clear(&mut self, color: (u8, u8, u8)) {
        for chunk in self.data.chunks_mut(3) {
            chunk[0] = color.0;
            chunk[1] = color.1;
            chunk[2] = color.2;
        }
    }

    /// Cambia el color de dibujo usado por point().
    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    /// Pinta un único píxel (x, y) con el color actual.
    /// Ignora silenciosamente coordenadas fuera de rango (aritmética entera,
    /// puede recibir valores negativos desde Bresenham en casos límite).
    pub fn point(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = (y * self.width + x) * 3;
        self.data[idx] = self.color.0;
        self.data[idx + 1] = self.color.1;
        self.data[idx + 2] = self.color.2;
    }
}
