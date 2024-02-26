use rpds::HashTrieMap;
use std::collections::{HashMap, HashSet};
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let n = env::var("N").unwrap().parse::<usize>().unwrap();
    let case = env::var("CASE").unwrap();

    match case.as_str() {
        "set_u64" => {
            let mut map = HashSet::with_capacity(n);
            for i in 0..n as u64 {
                map.insert(i);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            map.remove(&0);
        }

        "map_u64" => {
            let mut map = HashMap::with_capacity(n);
            for i in 0..n as u64 {
                map.insert(i, i);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            map.remove(&0);
        }

        "map_pkidx" => {
            let mut map = HashMap::with_capacity(n);
            for i in 0..n as u64 {
                let mut pk = [0u8; 48];
                pk[..8].copy_from_slice(&i.to_be_bytes());
                map.insert(pk, i);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            map.remove(&[0; 48]);
        }

        "rpds_map_pkidx" => {
            let mut map = HashTrieMap::new_sync();
            for i in 0..n as u64 {
                let mut pk = [0u8; 48];
                pk[..8].copy_from_slice(&i.to_be_bytes());
                map = map.insert(pk, i);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            map = map.remove(&[0; 48]);
            println!("{}", map.size());
        }

        _ => {}
    }
}
