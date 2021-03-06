// Lumol, an extensible molecular simulation engine
// Copyright (C) Lumol's contributors — BSD license

use lumol::sys::{System, TrajectoryBuilder};
use lumol_input::InteractionsInput;
use std::path::Path;

use rand::{XorShiftRng, SeedableRng};

pub fn get_system(name: &str) -> System {
    let data = Path::new(file!()).parent().unwrap().join("..").join("data");

    let mut system = TrajectoryBuilder::new()
                                       .open(data.join(String::from(name) + ".pdb"))
                                       .and_then(|mut trajectory| trajectory.read())
                                       .unwrap();

    InteractionsInput::new(data.join(String::from(name) + ".toml"))
                      .and_then(|input| input.read(&mut system))
                      .unwrap();

    return system;
}

pub fn get_rng(seed: u32) -> XorShiftRng {
    XorShiftRng::from_seed([seed, 784, 71255487, 5824])
}

macro_rules! benchmark_group {
    ($group_name:ident, $($function:path),+) => {
        pub fn $group_name() -> ::std::vec::Vec<bencher::TestDescAndFn> {
            use bencher::{TestDescAndFn, TestFn, TestDesc};
            use std::borrow::Cow;
            use std::path::Path;
            let mut benches = ::std::vec::Vec::new();
            $(
                let path = Path::new(file!());
                let path = path.file_stem().unwrap().to_string_lossy();
                benches.push(TestDescAndFn {
                    desc: TestDesc {
                        name: Cow::from(path + "::" + stringify!($function)),
                        ignore: false,
                    },
                    testfn: TestFn::StaticBenchFn($function),
                });
            )+
            benches
        }
    }
}
