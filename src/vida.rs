use crate::framebuffer::{Color, FrameBuffer};

fn esta_viva(fb: &FrameBuffer, x: i32, y: i32, ancho: i32, alto: i32) -> bool {
    // Borde toroidal: el mundo se envuelve sobre sí mismo, así los
    // patrones (como los gliders) nunca se topan con una pared.
    let x = x.rem_euclid(ancho);
    let y = y.rem_euclid(alto);
    fb.get_color(x, y) == Color::WHITE
}

fn contar_vecinos_vivos(fb: &FrameBuffer, x: i32, y: i32, ancho: i32, alto: i32) -> u8 {
    let mut vecinos = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if esta_viva(fb, x + dx, y + dy, ancho, alto) {
                vecinos += 1;
            }
        }
    }
    vecinos
}

/// Calcula y pinta la siguiente generación del juego de la vida de Conway.
/// No limpia el framebuffer: cada célula se vuelve a pintar (viva o
/// muerta) usando únicamente `point`, con base en lo leído por `get_color`.
pub fn siguiente_generacion(fb: &mut FrameBuffer) {
    let (ancho, alto) = fb.dimensions();
    let (ancho, alto) = (ancho as i32, alto as i32);

    let mut siguiente = vec![false; (ancho * alto) as usize];

    for y in 0..alto {
        for x in 0..ancho {
            let viva = esta_viva(fb, x, y, ancho, alto);
            let vecinos = contar_vecinos_vivos(fb, x, y, ancho, alto);

            let vivira = match (viva, vecinos) {
                (true, 2) | (true, 3) => true,  // survival
                (false, 3) => true,             // reproduction
                _ => false,                     // underpopulation / overpopulation
            };

            siguiente[(y * ancho + x) as usize] = vivira;
        }
    }

    for y in 0..alto {
        for x in 0..ancho {
            let vivira = siguiente[(y * ancho + x) as usize];
            fb.set_current_color(if vivira { Color::WHITE } else { Color::BLACK });
            fb.point(x, y);
        }
    }
}
