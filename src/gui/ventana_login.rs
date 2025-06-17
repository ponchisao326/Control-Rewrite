use fltk::{app, app::App, button::Button, frame::Frame, input::Input, prelude::*, window::Window};
use std::cell::RefCell;
use std::rc::Rc;
use fltk::enums::Color;

pub struct VentanaLogin {
    window: Window,
    result: Rc<RefCell<Option<String>>>,
}

impl VentanaLogin {
    /// Crea una nueva ventana de registro de usuario sobre la misma `app`
    pub fn new(app: &App) -> Self {
        // Crear ventana modal de 300x200
        let mut window = Window::new(
            (app::screen_size().0 as i32 - 300) / 2,
            (app::screen_size().1 as i32 - 200) / 2,
            300,
            200,
            "Registro de Usuario",
        );
        window.make_modal(true);

        // Campo Nombre
        let _ = Frame::new(20, 20, 60, 25, "Nombre:");
        let nombre_input = Input::new(90, 20, 180, 25, "");

        // Campo Apellido
        let _ = Frame::new(20, 60, 60, 25, "Apellido:");
        let apellido_input = Input::new(90, 60, 180, 25, "");

        // Label de error
        let mut error_frame = Frame::new(20, 100, 260, 25, "");
        error_frame.set_label_color(Color::Red);

        // Botón Iniciar
        let mut btn_iniciar = Button::new(100, 140, 100, 30, "Iniciar");

        // Compartir resultado entre callback y run
        let result: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let rclone = result.clone();
        let nombre_clone = nombre_input.clone();
        let apellido_clone = apellido_input.clone();
        let mut win_clone = window.clone();
        let mut err_clone = error_frame.clone();

        // Callback del botón
        btn_iniciar.set_callback(move |_| {
            let nombre = nombre_clone.value().trim().to_string();
            let apellido = apellido_clone.value().trim().to_string();
            if !nombre.is_empty() && !apellido.is_empty() {
                *rclone.borrow_mut() = Some(format!("{} {}", nombre, apellido));
                win_clone.hide();
            } else {
                err_clone.set_label("Por favor, complete todos los campos");
                win_clone.redraw();
            }
        });

        window.end();
        VentanaLogin { window, result }
    }

    /// Muestra la ventana modal y espera hasta que se cierra
    pub fn run(&mut self) -> Option<String> {
        self.window.show();
        // Loop modal sin añadir otro `app.run()`
        while self.window.shown() {
            app::wait();
        }
        self.result.borrow().clone()
    }
}
