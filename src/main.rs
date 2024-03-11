use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
enum EstadoConexion {
    Conectado,
    Desconectado,
}

fn run_process() -> Child {
    Command::new("flatpak")
        .args(["run", "-p", "org.videolan.VLC" ,"/home/gonik/Music/jaar.mp3"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("ACA CAGAMO")
}

fn main() {
    let mut process_that_i_want = run_process();
    println!("Process id {}", process_that_i_want.id());

    let (tx, rx) = mpsc::channel();
    let short_handle = thread::spawn(move || loop {
        let output = Command::new("curl")
            .args(["http://localhost:3000"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("No anduvo el comando");
        if !output.success() {
            tx.send(EstadoConexion::Desconectado).unwrap();
            println!("ESTADO ERROR");
            sleep(Duration::from_secs(2));
        } else {
            tx.send(EstadoConexion::Conectado).unwrap();
            // println!("ESTADO Feliz");
            sleep(Duration::from_secs(2));
        }
    });
    let long_handle = thread::spawn(move || {
        let mut estado_previo = EstadoConexion::Conectado;
        loop {
            let estado_actual = rx.recv().unwrap();
            if let EstadoConexion::Desconectado = estado_actual {
                println!("[ERROR]: no hay conexion");
                println!("Process id {}", process_that_i_want.id());
                process_that_i_want.kill().expect("No lo puedo cerrar");
               
            }
            if estado_previo == EstadoConexion::Desconectado
                && estado_actual == EstadoConexion::Conectado
            {
                process_that_i_want = run_process()
            }
            estado_previo = estado_actual;
        }
    });
    short_handle.join().unwrap();
    long_handle.join().unwrap();
    
}
