#[allow(unused_imports)]
use std::env;
use std::{path::{Path, PathBuf}, error::Error, process::{Command, Output}};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub struct It {
    pub binary: String,
    pub wait: bool,
    pub quiet_mode: bool,
    pub list_options: ListOptions,
}

#[derive(Debug)]
#[warn(dead_code)]
pub struct ListOptions {
    pub table: String,
    pub chain: String,
    pub verbose: bool,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref IPTABLESCHAIN: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("filter", vec!["INPUT", "FORWARD", "OUTPUT"]);
        map.insert("nat", vec!["PREROUTING", "OUTPUT", "POSTROUTING"]);
        map.insert("mangle", vec!["PREROUTING", "INPUT", "FORWARD", "OUTPUT", "POSTROUTING"]);
        map.insert("raw", vec!["INPUT", "FORWARD", "OUTPUT"]);
        map.insert("security", vec!["INPUT", "FORWARD", "OUTPUT"]);
        map
    };
}

pub fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

// return correct structs to support the target version of iptables (4 or 6)
pub fn build_it(itversion: i32, wait: bool, quiet_mode: bool, list_options: ListOptions) -> Option<It> {
    if itversion == 4 || itversion == 6 {
        let mut binary = "iptables";
        print!("{:?}", find_it(binary).unwrap());
        if itversion == 6 {
            binary = "ip6tables";
        }
        return Some(It {
            binary: String::from(binary),
            wait: wait,
            quiet_mode: quiet_mode,
            list_options:list_options,
        });
    }
    None
}

impl It {
    pub fn list_it(&mut self, tbl: String) -> Vec<String> {
        let mut v = vec!["-t".to_string(), tbl];
        if self.list_options.verbose {
            v.push("-v".to_string());
        }
        v.push("-L".to_string());
        if self.list_options.chain != "" {
            v.push(self.list_options.chain.to_string())
        }
        return v;
    }

    pub fn delete_chain(&mut self, tbl: String, chn: String) -> Result<(), Box<dyn Error>> {
        if is_builtin_chain(tbl, chn) {
            return Err("can\'t delete a built-in chain")?;
        } else{
            Ok(())
        }
    }

    pub fn run_command(&mut self) -> Output{
        let opts = ListOptions {
            table: self.list_options.table.to_string(),
            chain: self.list_options.chain.to_string(),
            verbose: false,
        };
        let bi = build_it(4, false, false, opts);
        let output = Command::new(find_it("iptables").unwrap())
            .args(bi.unwrap().list_it(self.list_options.table.to_string()))
            .output()
            .expect("failed to execute process");
        return output;
    }
}

pub fn is_builtin_chain(table: String, chain: String) -> bool {
    for (key, value) in &*IPTABLESCHAIN {
        if table == key.to_string() {
            for v in &*value {
                if v.to_string() == chain.to_string() {
                    print!("{:?}",v);
                    return true;
                }
            }        
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_builtin_chain_test() {
        let conditions = &[
            ("filter", "OUTPUT", true),
            ("filter", "OUTPUTmew", true),
        ];
        for (t, c,r )in conditions {
            let result = crate::is_builtin_chain(t.to_string(),c.to_string());
            println!("{:?}",result);
            assert_eq!(&result,r);
        }
    }

    fn delete_chain_test() {
        let conditions = &[
            ("filter", "OUTPUT", true),
            ("filter", "OUTPUTmew", true),
        ];
        for (t, c,r )in conditions {
            let result = crate::delete_chain(t.to_string(),c.to_string());
            println!("{:?}",result);
            assert_eq!(&result,r);
        }
    }
}
