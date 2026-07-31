mod framebuffer;
mod organismos;
mod vida;

use framebuffer::{Color, FrameBuffer};
use minifb::{Key, Window, WindowOptions};

fn poblacion_inicial(fb: &mut FrameBuffer) {
    // Fila superior: still lifes
    organismos::bloque(fb, 6, 6);
    organismos::colmena(fb, 14, 6);
    organismos::hogaza(fb, 22, 6);
    organismos::bote(fb, 30, 6);
    organismos::tina(fb, 38, 6);

    // Osciladores chicos
    organismos::parpadeador(fb, 6, 20);
    organismos::sapo(fb, 16, 20);
    organismos::faro(fb, 26, 20);
    organismos::parpadeador(fb, 36, 20);

    // Pulsares (osciladores grandes, periodo 3)
    organismos::pulsar(fb, 60, 4);
    organismos::pulsar(fb, 10, 80);

    // Naves viajando por el tablero
    organismos::planeador(fb, 6, 35);
    organismos::nave_ligera(fb, 20, 38);
    organismos::planeador(fb, 45, 35);
    organismos::nave_ligera(fb, 110, 20);
    organismos::planeador(fb, 120, 95);

    // Cañón: dispara un planeador nuevo cada 30 generaciones
    organismos::cañon_gosper(fb, 85, 45);

    // Methuselahs: patrones chicos que explotan en mucha actividad caótica
    organismos::r_pentomino(fb, 50, 60);
    organismos::bellota(fb, 90, 68);
    organismos::diehard(fb, 130, 60);
    organismos::r_pentomino(fb, 60, 105);
    organismos::diehard(fb, 20, 108);

    // Fila inferior: más still lifes y osciladores para llenar el tablero
    organismos::bloque(fb, 15, 95);
    organismos::colmena(fb, 30, 95);
    organismos::sapo(fb, 45, 95);
    organismos::faro(fb, 60, 95);
    organismos::parpadeador(fb, 75, 95);
    organismos::bote(fb, 90, 95);
    organismos::tina(fb, 105, 95);
}

fn main() {
    // Resolución baja para el grid de células; la ventana es más grande y
    // minifb estira el buffer para que cada célula se vea como un bloque.
    let ancho_grid = 160;
    let alto_grid = 120;
    let escala = 6;

    let mut framebuffer = FrameBuffer::new(ancho_grid, alto_grid, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);

    poblacion_inicial(&mut framebuffer);

    let mut window = Window::new(
        "Juego de la Vida",
        ancho_grid * escala,
        alto_grid * escala,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("no se pudo crear la ventana");

    // Un turno (generación) por frame; ~10 por segundo para poder verlo bien.
    window.set_target_fps(10);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // No se limpia el framebuffer: cada célula se vuelve a pintar
        // (viva o muerta) dentro de siguiente_generacion.
        vida::siguiente_generacion(&mut framebuffer);

        let (w, h) = framebuffer.dimensions();
        window
            .update_with_buffer(framebuffer.buffer(), w, h)
            .expect("no se pudo actualizar la ventana");
    }
}
