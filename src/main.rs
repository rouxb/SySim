
use structopt::StructOpt;
use rand::Rng;

pub mod hwt;

/// Define CLI arguments
#[derive(Debug, StructOpt)]
#[structopt(about = "Simulating Hw in Rust: PoC based on Rust-Async/ Tokio.")]
struct Opt {
    /// Number of coworker to generate
    #[structopt(short)]
    coworker: u32,

    /// Starting simulation tick
    #[structopt(long)]
    tick: usize,

    /// Number of tick per second (resolution)
    #[structopt(long)]
    timescale: usize,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("User Options:\n {:?}", opt);

    // Create and register the global TickKeeper
    let tk = hwt::TickKeeper::new(opt.tick, opt.timescale);
    hwt::register(tk);

    // Register tasks with random period
    // let mut rng = rand::thread_rng();
    // let tasks: Vec<hwt::HwTask>;
    // for t in 0..opt.coworker {
    //     let p = rng.gen_range(1..200);
    //     tasks.push(hwt::HwTask(format!("HwTask_P{}", p), p));
    //     tk.add_hwt();
    // }

    // Start simulation loop for 400 cycles
    // TickKeeper::global().simulate(400);
}
