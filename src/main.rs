use rpds::HashTrieMap;
use state_processing::AllCaches;
use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};
use types::{BeaconState, ChainSpec, EthSpec, MainnetEthSpec};

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

        "holesky_genesis" => {
            let mut map = HashMap::new();
            for i in 0..n {
                let (state, _) = read_holesky_state();
                map.insert(i, state);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            println!("{}", map.len());
        }

        "holesky_genesis_build_all_caches" => {
            let mut map = HashMap::new();
            for i in 0..n {
                let (mut state, spec) = read_holesky_state();
                state.build_all_caches(&spec).unwrap();
                map.insert(i, state);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            println!("{}", map.len());
        }

        "holesky_genesis_tree_cache" => {
            let mut map = HashMap::new();
            for i in 0..n {
                let (mut state, _) = read_holesky_state();
                state.update_tree_hash_cache().unwrap();
                map.insert(i, state);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            println!("{}", map.len());
        }

        "holesky_genesis_tree_cache_build_all_caches" => {
            let mut map = HashMap::new();
            for i in 0..n {
                let (mut state, spec) = read_holesky_state();
                state.build_all_caches(&spec).unwrap();
                state.update_tree_hash_cache().unwrap();
                map.insert(i, state);
            }

            println!("allocated");
            sleep(Duration::from_secs(1000));
            println!("{}", map.len());
        }

        _ => {}
    }
}

fn read_holesky_state() -> (BeaconState<MainnetEthSpec>, ChainSpec) {
    let state_filepath = env::var("BEACON_STATE_PATH").unwrap();
    let mut spec = MainnetEthSpec::default_spec();
    spec.altair_fork_epoch = Some(0u64.into());
    spec.bellatrix_fork_epoch = Some(0u64.into());

    (
        BeaconState::<MainnetEthSpec>::from_ssz_bytes(&fs::read(state_filepath).unwrap(), &spec)
            .unwrap(),
        spec,
    )
}
