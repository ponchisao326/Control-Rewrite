use fltk::{
    app,
    button::Button,
    enums::{Align, Color, FrameType},
    frame::Frame,
    group::{Flex, Group},
    prelude::*,
    window::Window,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn main_frame() {
    let app = app::App::default();

    // Obtener dimensiones de la pantalla
    let (screen_w, screen_h) = app::screen_size();
    let win_w = 200;
    let win_h = 250;
    let x_pos = screen_w as i32 - win_w - 10; // 10px de margen derecho
    let y_pos = screen_h as i32 - win_h - 40; // 40px de margen inferior

    // Crear ventana sin bordes y siempre visible
    let mut wind = Window::new(x_pos, y_pos, win_w, win_h, "");
    wind.set_border(false);
    wind.set_override();


    // Frame principal con borde gris
    let mut main_frame = Group::new(0, 0, win_w, win_h, "");
    main_frame.set_frame(FrameType::BorderBox);
    main_frame.set_color(Color::from_u32(0xD3D3D3)); // Gris claro

    // Contenedor principal vertical
    let mut col = Flex::default()
        .with_size(win_w - 4, win_h - 4) // Margen interior
        .column()
        .center_of_parent();
    col.set_margin(5);
    col.set_pad(5);

    // Frame de botones (horizontal)
    let mut btn_frame = Flex::default().row();
    let mut btn_connect = Button::default().with_label("Conectar");
    let mut btn_exit = Button::default().with_label("Salir");
    btn_frame.end();
    btn_frame.fixed(&btn_connect, 90); // Ancho fijo
    btn_frame.fixed(&btn_exit, 90);    // Ancho fijo

    // Frame de estado con texto
    let mut status_frame = Frame::default();
    status_frame.set_color(Color::Red);
    status_frame.set_frame(FrameType::BorderBox);
    status_frame.set_label("Desconectado");
    status_frame.set_label_color(Color::White);
    status_frame.set_label_size(12);
    status_frame.set_align(Align::Center);

    // Frame de reloj
    let mut clock_frame = Frame::default();
    clock_frame.set_label_size(20);
    clock_frame.set_label_color(Color::Black);

    col.end();
    col.fixed(&btn_frame, 30);      // Altura fija para botones
    col.fixed(&status_frame, 100);  // Altura fija para estado
    col.fixed(&clock_frame, 40);     // Altura fija para reloj

    main_frame.end();
    wind.end();
    wind.show();

    // Variables de estado
    let mut connected = false;
    let mut status_clone = status_frame.clone();
    let mut clock_clone = clock_frame.clone();
    let _btn_connect_clone = btn_connect.clone();

    // Callback para botón Conectar
    btn_connect.set_callback(move |b| {
        connected = !connected;

        if connected {
            b.set_label("Desconectar");
            status_clone.set_label("Conectado");
            status_clone.set_color(Color::Green);
            b.deactivate(); // Deshabilita el botón como en TKinter
        } else {
            b.set_label("Conectar");
            status_clone.set_label("Desconectado");
            status_clone.set_color(Color::Red);
        }
        status_clone.redraw();
    });

    // Callback para botón Salir
    btn_exit.set_callback(move |_| {
        app.quit();
    });

    // Actualización de posición y hora
    app::add_idle3(move |_| {
        // Mantener ventana en esquina inferior derecha
        let (screen_w, screen_h) = app::screen_size();
        let x_pos = screen_w as i32 - win_w - 10;
        let y_pos = screen_h as i32 - win_h - 40;
        wind.set_pos(x_pos, y_pos);

        // Actualizar hora
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hours = (now / 3600 % 24) as u8;
        let minutes = ((now / 60) % 60) as u8;
        clock_clone.set_label(&format!("{:02}:{:02}", hours, minutes));

        app::sleep(0.5); // Actualización más eficiente
    });

    app.run().unwrap();
}