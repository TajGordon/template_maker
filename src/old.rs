use std::{env, fs::File, io::Write};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        info_message();
    } else {
    let name = &args[1];
    if name == "--help" || name == "-h" {
        help_message();
    }
    else {
        let mut file = File::create(format!("{name}.cpp")).unwrap();

        let input_string = format!( "   freopen(\"{name}in.txt\", \"r\", stdin);\n");
        let output_string = format!("   freopen(\"{name}out.txt\", \"w\", stdout);\n");

        let start_string = String::from("#include \"bits/stdc++.h\"
#define F(I,J,K) for (int (I) = (J); (I) < (K); (I)++)
#define FR(I,J,K) for (int (I) = (K); (I) > (J); (I)--)
typedef long long ll;
typedef unsigned long long ull;
typedef long double ld;
using namespace std;
int main()
{
");

        let end_string = String::from("
    return 0;
}
        ");
        if args.len() > 2 {
            let arg = &args[2];
            if arg != "--aio" {
                let final_string = format!("{start_string} {end_string}");
                file.write_all(final_string.as_bytes()).unwrap();
            }
            else {
                let final_string = format!("{start_string} {input_string} {output_string} {end_string}");
                file.write_all(final_string.as_bytes()).unwrap();
            }
        }
        else {
            let final_string = format!("{start_string} {input_string} {output_string} {end_string}");
            file.write_all(final_string.as_bytes()).unwrap();
        }
    }}
}

fn info_message() {
    println!("  Template Maker (TM)");
    println!("-----------------------");
    println!("");
    println!("");
    println!("
This is a small tool\n
to build a template \n
C++ file for use in \n
the AIO
");
    println!("-----------------------");
}

fn help_message() {
    println!("  Template Maker (TM)");
    println!("-----------------------");
    println!("Just put the name of the");
    println!("problem as the file name");
    println!("and it will generate a");
    println!("C++ template file");
    println!("-----------------------");
}
