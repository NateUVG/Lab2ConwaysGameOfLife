use crate::framebuffer::{Color, FrameBuffer};

fn vivas(fb: &mut FrameBuffer, origen_x: i32, origen_y: i32, celulas: &[(i32, i32)]) {
    fb.set_current_color(Color::WHITE);
    for &(dx, dy) in celulas {
        fb.point(origen_x + dx, origen_y + dy);
    }
}

// --- Still lifes ---

pub fn bloque(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(fb, x, y, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn colmena(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)],
    );
}

pub fn hogaza(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)],
    );
}

pub fn bote(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(fb, x, y, &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]);
}

pub fn tina(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(fb, x, y, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}

// --- Oscillators ---

pub fn parpadeador(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(fb, x, y, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn sapo(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
    );
}

pub fn faro(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[
            (0, 0),
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 2),
            (2, 3),
            (3, 3),
        ],
    );
}

pub fn pulsar(fb: &mut FrameBuffer, x: i32, y: i32) {
    let brazo = [2, 3, 4];
    let filas_horizontales = [0, 5, 7, 12];
    let mut celulas = Vec::new();

    for &fila in &filas_horizontales {
        for &col in &brazo {
            celulas.push((col, fila));
            celulas.push((col + 6, fila));
        }
    }

    let columnas_verticales = [0, 5, 7, 12];
    for &col in &columnas_verticales {
        for &fila in &brazo {
            celulas.push((col, fila));
            celulas.push((col, fila + 6));
        }
    }

    vivas(fb, x, y, &celulas);
}

// --- Spaceships ---

pub fn planeador(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
    );
}

pub fn nave_ligera(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[
            (1, 0),
            (4, 0),
            (0, 1),
            (0, 2),
            (4, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
        ],
    );
}

// --- Methuselahs (patrones chicos que generan mucho caos disperso) ---

pub fn r_pentomino(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(fb, x, y, &[(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]);
}

pub fn bellota(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
    );
}

pub fn diehard(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)],
    );
}

// --- Gun ---

/// Gosper Glider Gun: dispara un planeador nuevo cada 30 generaciones.
pub fn cañon_gosper(fb: &mut FrameBuffer, x: i32, y: i32) {
    vivas(
        fb,
        x,
        y,
        &[
            // bloque izquierdo
            (0, 4),
            (1, 4),
            (0, 5),
            (1, 5),
            // estructura izquierda
            (10, 4),
            (10, 5),
            (10, 6),
            (11, 3),
            (11, 7),
            (12, 2),
            (12, 8),
            (13, 2),
            (13, 8),
            (14, 5),
            (15, 3),
            (15, 7),
            (16, 4),
            (16, 5),
            (16, 6),
            (17, 5),
            // estructura derecha
            (20, 2),
            (20, 3),
            (20, 4),
            (21, 2),
            (21, 3),
            (21, 4),
            (22, 1),
            (22, 5),
            (24, 0),
            (24, 1),
            (24, 5),
            (24, 6),
            // bloque derecho
            (34, 2),
            (35, 2),
            (34, 3),
            (35, 3),
        ],
    );
}
