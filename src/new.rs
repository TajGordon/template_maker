use std::{env, fs::File, io::Write};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    /* If not given an argument, there is always 1 from sys */
    if args.len() < 2 {
        info_message();
    }

    let mut message: String = String::from("#include <bits/stdc++.h>
#define F(I,J,K) for (int (I) = (J); (I) < (K); (I)++)
#define FR(I,J,K) for (int (I) = (K); (I) > (J); (I)--)
typedef long long ll;
typedef unsigned long long ull;
typedef long double ld;
using namespace std;
int main()
{
");

    let mut no_aio: bool = false;
    let mut name: String = String::new();

    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => help_message(),

            "--no-aio" => no_aio = true,

            /* Do nothing about invalid arguments */
            _ => name = arg,
        }
    }

    if !no_aio {
        message.push_str(format!("    #ifndef NO_AIO\n    freopen(\"{name}in.txt\", \"r\", stdin);\n    freopen(\"{name}out.txt\", \"w\", stdout);\n    #endif\n").as_str());
    }
    message.push_str("}");

    let mut file = File::create(format!("{name}.cpp")).unwrap();
    file.write_all(message.as_bytes()).unwrap();
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
    let message = String::from("usage: teml [filename] --[args]");
    println!("{message}");
}
