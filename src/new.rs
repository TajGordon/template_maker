use std::{env, fs::File, io::Write, process::{Command, Stdio}};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    /* If not given an argument, there is always 1 from sys */
    if args.len() > 1 {
        if args[1] == "run" {
            if args.len() < 3 {
                println!("error: no input file given");
            } else {
                let compile_output = Command::new("c++")
                    .arg(format!("{}.cpp", args[2]))
                    .arg("-o")
                    .arg(format!("{}", args[2]))
                    .arg("-DNO_AIO")
                    .output()
                    .expect("Failed to execute compile command");
                if compile_output.status.success() {
                    let mut child = Command::new(format!("./{}", args[2]))
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("Failed to execute file");
                    println!("Started process with PID: {}", child.id());

                    let exit_status = child.wait().expect("Failed to wait for child");
                    if exit_status.success() {
                        println!("Program finished successfully");
                    } else {
                        println!("Program finished successfully");
                    }
                }
            }
        } if args[1] == "new" || args[1] == "add" {
            let message: String = String::from("#include <bits/stdc++.h>
using namespace std;
#define int long long
#define inf LONG_LONG_MAX
signed main()
{
}
");

            // let mut no_aio: bool = false;
            let mut name: String = String::new();

            for arg in args.iter().skip(2) {
                match arg.as_str() {
                    "--help" | "-h" => help_message(),

                    // "--no-aio" => no_aio = true,

                    "add" | "new" => (),
                    /* Do nothing about invalid arguments */
                    _ => name = arg.clone(),
                }
            }

            if name == String::new() {
                help_message();
            } else {
                // if !no_aio {
                //     message.push_str(format!("    #ifndef NO_AIO\n    freopen(\"{name}in.txt\", \"r\", stdin);\n    freopen(\"{name}out.txt\", \"w\", stdout);\n    #endif\n").as_str());
                // }
                // message.push_str("}");

                let mut file = File::create(format!("{name}.cpp")).unwrap();
                file.write_all(message.as_bytes()).unwrap();
            }
        } else {
            help_message();
        }
    } else {
        info_message();
    }
}

fn info_message() {
    let message = String::from("        Template Maker (TEML)
--------------------------------------
        This is a small tool
        to build a template
        C++ file for the AIO
--------------------------------------");
    println!("{message}");
}

fn help_message() {
    let message = String::from("usage: teml add [filename] --[args]\n       teml run [filename]");
    println!("{message}");
}
