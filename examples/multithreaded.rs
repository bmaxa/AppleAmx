use amx::{prelude::*, XBytes, YBytes, ZRow};
//use clap::Clap;
use std::time::Instant;
/*
#[derive(Debug, Clap)]
struct Opts {
    /// Number of threads to launch
    #[clap(short = 'n')]
    num_threads: usize,
}
*/
fn main() {
  //  let opts: Opts = Clap::parse();
    println!("Launching {} threads with AMX enabled", 1);

    for i in 1..1 {
        std::thread::spawn(move || stress_loop(i));
    }
    stress_loop(0);
}

#[inline(never)]
fn stress_loop(tid: usize) {
    let mut ctx = amx::AmxCtx::new().unwrap();

    loop {
        let start = Instant::now();
        let count = 10_000_000;
        for _ in 0..count / 16 {
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);

            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(0), true);
            ctx.outer_product_i16_xy_to_z(Some(XBytes(0)), Some(YBytes(0)), ZRow(1), true);
        }
        let rate = count as f64 / start.elapsed().as_secs_f64();
        println!("[{:3}] {:2} amxmac16s per second", tid, rate);
    }
}
