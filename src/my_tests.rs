use amx::{prelude::*, XRow, YRow, ZRow};
extern {
  fn init_time()->u64;
  fn time_me(tm:u64)->f64;
}
fn main() {
    unsafe {
        let mut ctx = amx::AmxCtx::new().unwrap();

        let _in_x: Vec<u16> = vec![1;256];
        let _in_y: Vec<u16> = vec![3;256];
        let mut in_xf: Vec<f64> = vec![1.0;64];
        let mut in_yf: Vec<f64> = vec![3.0;64];
        let in_zf: Vec<f64> = vec![2.0;64*8];
        for i in 0..64 {
          for _ in 0..8{
            in_xf[i] += i as f64;
            in_yf[i] += i as f64;
          }
        }
        ctx.clear();
        ctx.set0();

        for i in 0..8 {
            //ctx.load512(&in_x[i * 32], XRow(i));
            //ctx.load512(&in_y[i * 32], YRow(i));
            ctx.load512(&in_xf[i*8], XRow(i));
            ctx.load512(&in_yf[i*8], YRow(i));
        }
        for i in 0..64 {
            ctx.load512(&in_zf[i*8], ZRow(i));
        }

//        println!("x = {:?}", *(in_x.as_ptr() as *const [[u16; 32]; 8]));
//        println!("y = {:?}", *(in_y.as_ptr() as *const [[u16; 32]; 8]));
            ctx.extr_yh(5,5);
       let got_x = std::mem::transmute::<_,[[f64;8];8]>(ctx.read_x());
       let got_y = std::mem::transmute::<_,[[f64;8];8]>(ctx.read_y());
       println!("X");
       print_a::<8,8>(&got_x);
       println!("Y");
       print_a::<8,8>(&got_y);
            let mut two:[f64;8] = [2.0;8];
            let two1 = two;
            //ctx.fma64_vec(0,0,0,0);
            //ctx.fma64_vec(7,7,7,0);
            let tm = init_time();
            let mut sum = 0.0;
            for i in 0..1000000 {

             for j in 0..8 {
                two[j]=(i*8+j+1) as f64;
              }
              ctx.load512(&two,ZRow(50));
              ctx.rcp(50,51);
              ctx.sqrt(50,52);
              ctx.extr_xh(51,0);
              ctx.extr_yh(52,0);
              ctx.fma64_vec_xy(51,0,0,0);
              ctx.store512(&mut two,ZRow(51));

              //ctx.fma64_mat(0,0,0,0);
              //ctx.fma32_mat(0,0,0,0);
              //ctx.fma16_mat(0,0,0,0);
              //ctx.extr_xh(0,0);
              //ctx.extr_yh(0,0);
              //ctx.fma64_vec_x(0,0);
              sum+=two.iter().sum::<f64>();
            }
            let res = time_me(tm);
            println!("simd time {:.18} sum {}",res, sum);
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.fma64_vec(0,0,0,0);
            }
            println!("single fma64_vec {}",time_me(tm));
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.fma32_vec(0,0,0,0);
            }
            println!("single fma32_vec {}",time_me(tm));
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.fma16_vec(0,0,0,0);
            }
            println!("single fma16_vec {}",time_me(tm));
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.extr_xh(0,0);
            }
            println!("single extr_xh {}",time_me(tm));
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.extr_yh(0,0);
            }
            println!("single extr_yh {}",time_me(tm));
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.load512(&two,ZRow(0));
            }
            println!("single load512 {}",time_me(tm));
            let tm = init_time();
            for _ in 0..1000000 {
              ctx.store512(&mut two,ZRow(0));
            }
            println!("single store512 {}",time_me(tm));

            let tm = init_time();
            for _ in 0..1000000 {
              ctx.int32_mat(
                0,
                0,
                0
              );
            }
            println!("single int32_mat {}",time_me(tm));

            //ctx.reduce_u32_to_z();

            let mut sum = 0.0;
            for i in 0..8000000 {
             // for v in two1 {
                sum+=((i+1)as f64).sqrt()*1.0/((i+1) as f64);
              //}
            }
            let res = time_me(tm);
            println!("seq time {} sum {}",res,sum);
            ctx.load512(&two1,ZRow(50));
            ctx.sqrt(50,51);
            ctx.store512(&mut two,ZRow(51));
            println!("sqrt\n{:?}",two);
            let sqrt=two1[0].sqrt();
            println!("sqrtseq\n{:?}",sqrt);
            let mut rcp = [2.0;8];
            ctx.load512(&rcp,ZRow(63));
            ctx.rcp(63,63);
            ctx.store512(&mut rcp,ZRow(63));
            println!("rcp\n{:?}",rcp);


//            let got_z = std::mem::transmute::<_,[[u32;16];64]>(ctx.read_z());
            ctx.fma64_mat_x(3,0);
            ctx.fma64_mat_y(5,1);
            ctx.fma64_mat(3,0,0,1);
            ctx.extr_xv(3,7);
            let got_y = std::mem::transmute::<_,[[f64;8];8]>(ctx.read_x());
            println!("X");
            print_a::<8,8>(&got_y);
            let got_z = std::mem::transmute::<_,[[f64;8];64]>(ctx.read_z());
            println!("Z");
            print_a::<64,8>(&got_z);

    }
}
fn print_a<const ROWS:usize,const COLS:usize>(a:&[[f64;COLS];ROWS]){
  for i in 0..ROWS {
    println!("{:?}", a[i])
  }
}
