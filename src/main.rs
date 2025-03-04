use std::process::Stdio;
use procfs::process::all_processes;

fn main() {

    let is_server_running = all_processes().unwrap()
        .any(|process| {
            if let Ok(process) = process {
                if let Ok(cmdline) = process.cmdline() {
                    if !cmdline.is_empty() && cmdline[0].contains("axum_server") {
                        return true;
                    }
                }
            }

            return false;
        });

    let _server = if !is_server_running {
        let cwd = std::env::current_dir().unwrap();
        if !cwd.join("assignment2-tester").exists() {
            println!("Please run the tester from its workspace directory");
            std::process::exit(1);
        }
        let command_cwd = cwd
            .join("target")
            .join("debug");

        let server = std::process::Command::new("./axum_server")
            .current_dir(command_cwd)
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to start server"); 

        std::thread::sleep(std::time::Duration::from_secs(10));

        Some(server)
    } else {
        None
    };

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
        let results = tm.run_tests();
        println!();
        println!("----- Tests Results -----");
        for (name, ok) in results {
            if ok {
                println!("[+] {}... \x1b[32mok\x1b[0m", name);
            } else {
                println!("[-] {}... \x1b[31mfailed\x1b[0m", name);
            }
        }
    } else {
        println!("Failed to compile assignment");
    }

    // keep the server running...
    // _server.unwarp().kill().unwrap();
}
