mod gui;
mod utils;
mod encrypter;

fn main() {
    //gui::main_frame::main_frame();

    /*
    let password = "1IY@AkB!Hi^Y2q6>K{)RgLw";
    let logger = encrypter::log_encryption::LogEncryption::new(password);

    let mensaje = "Este es un mensaje secreto";
    let token = logger.encrypt(mensaje);
    println!("Token cifrado: {}", token);

    match logger.decrypt(&token) {
        Ok(pt) => println!("Mensaje descifrado: {}", pt),
        Err(_) => println!("Fallo en la verificación de autenticidad"),
    } */

    // Prueba de Logger
    let logger = utils::loggers::Logger::setup("Juan Garcia Perez");
    logger.log_with_flush("procesos", "Este es un mensaje de prueba");
    logger.log_alert("IA", "Uso sospechoso de IA detectado", "WARNING");
}