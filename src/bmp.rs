use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Write};

/// Escribe el framebuffer a disco como un archivo BMP de 24 bits sin comprimir.
/// Implementado a mano (sin dependencias externas) siguiendo el formato
/// clásico BITMAPFILEHEADER + BITMAPINFOHEADER + datos de píxeles.
pub fn write_bmp(fb: &Framebuffer, path: &str) -> io::Result<()> {
    let width = fb.width as i32;
    let height = fb.height as i32;

    // BMP almacena filas de abajo hacia arriba y cada fila debe estar
    // alineada a un múltiplo de 4 bytes ("padding").
    let row_size = (width * 3 + 3) & !3;
    let padding = row_size - width * 3;
    let pixel_data_size = row_size * height;

    let file_header_size = 14u32;
    let info_header_size = 40u32;
    let pixel_data_offset = file_header_size + info_header_size;
    let file_size = pixel_data_offset + pixel_data_size as u32;

    let mut buf: Vec<u8> = Vec::with_capacity(file_size as usize);

    // --- BITMAPFILEHEADER (14 bytes) ---
    buf.extend_from_slice(b"BM");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(&0u16.to_le_bytes()); // reserved1
    buf.extend_from_slice(&0u16.to_le_bytes()); // reserved2
    buf.extend_from_slice(&pixel_data_offset.to_le_bytes());

    // --- BITMAPINFOHEADER (40 bytes) ---
    buf.extend_from_slice(&info_header_size.to_le_bytes());
    buf.extend_from_slice(&width.to_le_bytes());
    buf.extend_from_slice(&height.to_le_bytes()); // positivo => bottom-up
    buf.extend_from_slice(&1u16.to_le_bytes()); // planes
    buf.extend_from_slice(&24u16.to_le_bytes()); // bits por píxel
    buf.extend_from_slice(&0u32.to_le_bytes()); // sin compresión (BI_RGB)
    buf.extend_from_slice(&(pixel_data_size as u32).to_le_bytes());
    buf.extend_from_slice(&2835i32.to_le_bytes()); // ~72 DPI horizontal
    buf.extend_from_slice(&2835i32.to_le_bytes()); // ~72 DPI vertical
    buf.extend_from_slice(&0u32.to_le_bytes()); // colores en paleta
    buf.extend_from_slice(&0u32.to_le_bytes()); // colores importantes

    // --- Datos de píxeles (BGR, de abajo hacia arriba) ---
    for y in (0..fb.height).rev() {
        for x in 0..fb.width {
            let idx = (y * fb.width + x) * 3;
            let r = fb.data[idx];
            let g = fb.data[idx + 1];
            let b = fb.data[idx + 2];
            buf.push(b);
            buf.push(g);
            buf.push(r);
        }
        for _ in 0..padding {
            buf.push(0);
        }
    }

    let mut file = File::create(path)?;
    file.write_all(&buf)?;
    Ok(())
}
