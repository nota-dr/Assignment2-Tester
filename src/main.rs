use libc::{prctl, PR_SET_PDEATHSIG, SIGHUP};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut server = unsafe {
        Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("assignment2-tester")
            .arg("--bin")
            .arg("axum_server")
            .stdout(Stdio::null())
            .pre_exec(|| {
                prctl(PR_SET_PDEATHSIG, SIGHUP);
                Ok(())
            })
            .spawn()
            .expect("Failed to start server")
    };

    sleep(Duration::from_secs(10));

    match std::process::Command::new("valgrind")
        .arg("--version")
        .output()
    {
        Ok(output) if output.status.success() => {
            println!("[+] Valgrind is installed.");
            println!(
                "[+] Version: {}",
                String::from_utf8_lossy(&output.stdout)
            );
            println!()
        }
        Ok(_) => {
            println!(
                "[+] Valgrind command found, but it failed to run properly."
            );
            std::process::exit(1);
        }
        Err(_) => {
            println!("[-] Valgrind is not installed or not in PATH Please install it.");
            std::process::exit(1);
        }
    }

    let mut tm = tests_lib::TestManager::new("assignment2", "testee", 0);

    assignment2_tester::add_tests(&mut tm);

    let compilation = tm.compile_assignment("gcc -Wall *.c -o client");
    if compilation != "error" {
        println!("----- Tests Results -----");
        for (name, ok) in tm.run_tests() {
            if ok {
                println!("[+] {}... \x1b[32mok\x1b[0m", name);
            } else {
                println!("[-] {}... \x1b[31mfailed\x1b[0m", name);
            }
        }
    } else {
        println!("Failed to compile assignment");
    }

    server.kill().unwrap();
}
