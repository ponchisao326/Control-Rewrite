use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use chrono::Local;

pub struct Logger {
    pub loggers: std::collections::HashMap<String, String>,
    pub nombre_estudiante: String,
}

impl Logger {
    pub fn setup(nombre_completo: &str) -> Self {
        let logs_dir = "logs";
        let _ = create_dir_all(logs_dir);
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let nombre_archivo = nombre_completo.replace(" ", "_");
        let log_types = vec![
            ("actividad", "actividad"),
            ("red", "red"),
            ("procesos", "procesos"),
            ("alertas", "alertas_ia"),
        ];
        let mut loggers = std::collections::HashMap::new();
        for (log_key, log_name) in log_types {
            let file_path = format!("{}/{}_{}_{}.log", logs_dir, log_name, nombre_archivo, timestamp);
            loggers.insert(log_key.to_string(), file_path);
        }
        let logger = Logger {
            loggers,
            nombre_estudiante: nombre_completo.to_string(),
        };
        logger.init_alertas();
        logger
    }

    fn init_alertas(&self) {
        if let Some(alertas_path) = self.loggers.get("alertas") {
            let inicio = format!(
                "=== INICIO DE MONITOREO DE ALERTAS DE IA ===\nUsuario: {}\nFecha y hora de inicio: {}\n==================================================",
                self.nombre_estudiante,
                Local::now()
            );
            self.log_with_flush("alertas", &inicio);
        }
    }

    pub fn log_with_flush(&self, log_type: &str, message: &str) {
        if let Some(file_path) = self.loggers.get(log_type) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_message = format!("{} - {}\n", timestamp, message);
            if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(file_path) {
                let _ = file.write_all(log_message.as_bytes());
                let _ = file.flush();
            }
        }
    }

    pub fn log_alert(&self, alert_type: &str, message: &str, severity: &str) {
        let formatted = format!("[{}] [{}] {}", severity, alert_type, message);
        self.log_with_flush("alertas", &formatted);
    }
}