use iptables_rs_lib;

// example of using iptables-rs-lib
fn main() {
    let opts = iptables_rs_lib::ListOptions {
        table: String::from("mangle"),
        chain: String::from("INPUT"),
        verbose: true,
    };
    let bi = iptables_rs_lib::build_it(6, false, false, opts);
    let output = bi.unwrap().run_command();
    println!("{:?}", output);
}
