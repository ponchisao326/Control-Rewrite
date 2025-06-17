use fltk::{
    app::{self, App},
    button::Button,
    enums::{Align, Color, FrameType},
    frame::Frame,
    group::{Flex, Group},
    prelude::*,
    window::Window,
};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::gui::ventana_login::VentanaLogin;

pub fn main_frame() {
    // Inicializa la aplicación FLTK una sola vez
    let app = App::default();

    // Calcula dimensiones y posición para mostrar la ventana en la esquina inferior derecha
    let (screen_w, screen_h) = app::screen_size();
    let win_w = 200;
    let win_h = 200;
    let x_pos = screen_w as i32 - win_w - 10;
    let y_pos = screen_h as i32 - win_h - 40;

    // Crea la ventana principal sin bordes
    let mut wind = Window::new(x_pos, y_pos, win_w, win_h, "");
    wind.set_border(false);

    // Grupo principal con fondo gris claro
    let mut main_frame = Group::new(0, 0, win_w, win_h, "");
    main_frame.set_frame(FrameType::BorderBox);
    main_frame.set_color(Color::from_u32(0xD3D3D3));

    // Flexbox vertical para organizar los elementos
    let mut col = Flex::default()
        .with_size(win_w - 4, win_h - 4)
        .column()
        .center_of_parent();
    col.set_margin(5);
    col.set_pad(5);

    // Fila de botones: Conectar y Salir
    let mut btn_frame = Flex::default().row();
    let mut btn_connect = Button::default().with_label("Conectar");
    let mut btn_exit = Button::default().with_label("Salir");
    btn_frame.end();
    btn_frame.fixed(&btn_connect, 90);
    btn_frame.fixed(&btn_exit, 90);

    // Cuadro de estado (rojo por defecto, "Desconectado")
    let mut status_frame = Frame::default();
    status_frame.set_color(Color::Red);
    status_frame.set_frame(FrameType::BorderBox);
    status_frame.set_label("Desconectado");
    status_frame.set_label_color(Color::White);
    status_frame.set_label_size(12);
    status_frame.set_align(Align::Center);

    // Reloj digital
    let mut clock_frame = Frame::default();
    clock_frame.set_label_size(20);
    clock_frame.set_label_color(Color::Black);

    col.end();
    col.fixed(&btn_frame, 30);
    col.fixed(&status_frame, 100);
    col.fixed(&clock_frame, 40);

    main_frame.end();
    wind.end();

    // Mostramos la ventana antes de manipular el handle nativo
    wind.show();

    // Forzamos que la ventana esté siempre encima y sin decoraciones
    wind.set_override();
    wind.set_on_top();

    // Callback para el botón Conectar
    let mut status_clone = status_frame.clone();
    btn_connect.set_callback(move |_| {
        // Abre ventana de login modal usando la misma app
        let mut login = VentanaLogin::new(&app);
        match login.run() {
            Some(nombre) => {
                println!("Usuario registrado: {}", nombre);
                status_clone.set_label("Conectado");
                status_clone.set_color(Color::Green);
            }
            None => {
                status_clone.set_label("Desconectado");
                status_clone.set_color(Color::Red);
            }
        }
        status_clone.redraw();
    });

    // Callback para el botón Salir
    btn_exit.set_callback(move |_| app.quit());

    // Idle callback: actualiza posición de ventana y reloj cada 0.5s
    let mut clock_clone = clock_frame.clone();
    app::add_idle3(move |_| {
        let (sw, sh) = app::screen_size();
        wind.set_pos(sw as i32 - win_w - 10, sh as i32 - win_h - 40);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hours = (now / 3600 % 24) as u8;
        let minutes = ((now / 60) % 60) as u8;
        clock_clone.set_label(&format!("{:02}:{:02}", hours, minutes));

        app::sleep(0.5);
    });

    // Ejecuta el bucle principal de la app
    app.run().unwrap();
}