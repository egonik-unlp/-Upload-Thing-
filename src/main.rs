use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;



const TARGET_PROGRAM : &str = "flatpak";
const TARGET_ARGS : [&str; 4] = ["run", "-p", "org.videolan.VLC" ,"/home/gonik/Music/jaar.mp3"];
const LISTENER_PROGRAM : &str = "curl";
const LISTENER_ARGS : [&str; 1] = ["http://localhost:3000"];


#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnectionStatus {
    Connected,
    Disconnected,
}


fn run_process() -> Child {
    Command::new(TARGET_PROGRAM)
        .args(TARGET_ARGS)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Can't launch target process")
}

fn main() {
    let mut process_that_i_want = run_process();
    println!("Process with id {} launched", process_that_i_want.id());

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let output = Command::new(LISTENER_PROGRAM)
            .args(LISTENER_ARGS)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Can't run listener command");
        if !output.success() {
            tx.send(ConnectionStatus::Disconnected).unwrap();
            sleep(Duration::from_secs(2));
        } else {
            tx.send(ConnectionStatus::Connected).unwrap();
            sleep(Duration::from_secs(2));
        }
    });
    let mut previous_state = ConnectionStatus::Connected;
    loop {
        let current_state = rx.recv().unwrap();
        if let ConnectionStatus::Disconnected = current_state {
            if previous_state == ConnectionStatus::Connected {
                println!("[DISCONNECTED]: Listener ping received no response from remote url");
            }
            process_that_i_want.kill().expect("Process cannot be killed");
           
        }
        if previous_state == ConnectionStatus::Disconnected
            && current_state == ConnectionStatus::Connected
        {
            println!("Process restarted with PID = {}", process_that_i_want.id());

            process_that_i_want = run_process();
        }
        previous_state = current_state;
    }    
}
