use amx::{prelude::*, XRow, YRow, ZRow,XBytes,Reverse,Normal,Index4,F64,X64};
const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const YEAR: f64 = 365.24;
const N_BODIES: usize = 5;
extern {
  fn init_time()->u64;
  fn time_me(tm:u64)->f64;
}
static mut X:[f64;16] = [0.0,4.84143144246472090e+00,8.34336671824457987e+00,1.28943695621391310e+01,1.53796971148509165e+01,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static mut Y:[f64;16] = [0.0,-1.16032004402742839e+00,4.12479856412430479e+00,-1.51111514016986312e+01,-2.59193146099879641e+01,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static mut Z:[f64;16] = [0.0,-1.03622044471123109e-01,-4.03523417114321381e-01,-2.23307578892655734e-01,1.79258772950371181e-01,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static mut VX:[f64;16] = [0.0,1.66007664274403694e-03 * YEAR,-2.76742510726862411e-03 * YEAR,2.96460137564761618e-03 * YEAR,2.68067772490389322e-03 * YEAR,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static mut VY:[f64;16] = [0.0,7.69901118419740425e-03 * YEAR,4.99852801234917238e-03 * YEAR,2.37847173959480950e-03 * YEAR,1.62824170038242295e-03 * YEAR,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static mut VZ:[f64;16] = [0.0,-6.90460016972063023e-05 * YEAR,2.30417297573763929e-05 * YEAR,-2.96589568540237556e-05 * YEAR,-9.51592254519715870e-05 * YEAR,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static mut MASS:[f64;16] = [SOLAR_MASS,9.54791938424326609e-04 * SOLAR_MASS,2.85885980666130812e-04 * SOLAR_MASS,4.36624404335156298e-05 * SOLAR_MASS,5.15138902046611451e-05 * SOLAR_MASS,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
static INDEXES:[u64;8] = [1,2,3,4,5,6,7,0];
static VALUESU:[u64;8] = [0,1,2,3,4,5,6,7];

unsafe fn advance(dt: f64, steps: i32) {
    let mut tmp_vx = [0.0;8];
    let mut tmp_vy = [0.0;8];
    let mut tmp_vz = [0.0;8];
    let dt = [dt;8];
    let mut ctx = amx::AmxCtx::new().unwrap();
    ctx.load512(&dt,ZRow(5));
    for _ in 0 .. steps {
        ctx.load512(&X,YRow(4));
        ctx.load512(&Y,YRow(5));
        ctx.load512(&Z,YRow(6));
        ctx.load512(&MASS,YRow(7));
        ctx.load512(&INDEXES,XRow(0));
        ctx.load512(&VALUESU,XRow(1));
        ctx.lut(XBytes(0),XRow(1),XRow(2),(Reverse,Index4,F64));
        ctx.fma64_vec_x(27,2);
        ctx.fma64_vec_y(28,4);
        ctx.fma64_vec_y(29,5);
        ctx.fma64_vec_y(30,6);
        for i in 0..N_BODIES as u64 {
            ctx.fma64_mat_y(0,4);
            ctx.fma64_mat_y(1,5);
            ctx.fma64_mat_y(2,6);
            ctx.extr_xh(27,7);
            ctx.extr_xh(28,3);
            ctx.lut(XBytes(7*64),XRow(3),XRow(0),(Normal,Index4,X64));
            ctx.fma64_vec_x(28,0);
            ctx.extr_xh(29,3);
            ctx.lut(XBytes(7*64),XRow(3),XRow(1),(Normal,Index4,X64));
            ctx.fma64_vec_x(29,1);
            ctx.extr_xh(30,3);
            ctx.lut(XBytes(7*64),XRow(3),XRow(2),(Normal,Index4,X64));
            ctx.fma64_vec_x(30,2);
            ctx.fms64_vec_xz(i*8+0,0);//dx
            ctx.fms64_vec_xz(i*8+1,1);//dy
            ctx.fms64_vec_xz(i*8+2,2);//dz
            ctx.extr_xh(i*8+0,7);
            ctx.fma64_vec_x(40,7);// dx->Z[40]
            ctx.extr_xh(i*8+1,7);
            ctx.fma64_vec_x(41,7);// dy->Z[41]
            ctx.extr_xh(i*8+2,7);
            ctx.fma64_vec_x(42,7);// dz->Z[42]
            ctx.extr_xh(i*8+0,0);
            ctx.extr_yh(i*8+0,8);
            ctx.fma64_vec_xy(0,0,0,0);
            ctx.extr_xh(i*8+1,1);
            ctx.extr_yh(i*8+1,1);
            ctx.fma64_vec_xy(1,1,1,0);
            ctx.extr_xh(i*8+2,2);
            ctx.extr_yh(i*8+2,2);
            ctx.fma64_vec_xy(2,2,2,0);
            ctx.extr_xh(1,0);
            ctx.extr_yh(2,0);
            ctx.fma64_vec_xz(0,0);
            ctx.fma64_vec_yz(0,0);// dx^2+dy^2+dz^2
            ctx.extr_xh(0,1);
            ctx.fma64_vec_x(50,1);
            ctx.sqrt(50,51);
            ctx.extr_yh(51,0);
            ctx.extr_xh(50,0);
            ctx.fma64_vec_xy(0,0,0,0);
            ctx.extr_xh(0,0);
            ctx.fma64_vec_x(50,0);
            ctx.rcp(50,51);
            ctx.extr_xh(5,0);
            ctx.extr_yh(51,0);
            ctx.fma64_vec_xy(0,0,0,0);//mag
            ctx.load512(&MASS[i as usize +1],XRow(3));
            ctx.extr_yh(0,3);
            ctx.fma64_vec_y(6,3);// mag -> Z[6]
            ctx.fma64_vec_xy(3,3,3,0);// massj_mag
            ctx.extr_yh(3,3);
            ctx.extr_xh(40,0);
            ctx.extr_xh(41,1);
            ctx.extr_xh(42,2);
            ctx.fms64_vec_xy(60,0,3,N_BODIES as u64 - i - 1);
            ctx.fms64_vec_xy(61,1,3,N_BODIES as u64 - i - 1);
            ctx.fms64_vec_xy(62,2,3,N_BODIES as u64 - i - 1);
            ctx.store512(&mut tmp_vx,ZRow(60));
            ctx.store512(&mut tmp_vy,ZRow(61));
            ctx.store512(&mut tmp_vz,ZRow(62));
            VX[i as usize] += tmp_vx.iter().take(N_BODIES -i as usize - 1).sum::<f64>();
            VY[i as usize] += tmp_vy.iter().take(N_BODIES -i as usize - 1).sum::<f64>();
            VZ[i as usize] += tmp_vz.iter().take(N_BODIES -i as usize - 1).sum::<f64>();
            ctx.fma64_mat_y(7,7);
            ctx.extr_xh(i*8+7,3);
            ctx.extr_yh(6,3);// mag -> Y[3]
            ctx.fma64_vec_xy(3,3,3,0);// massi_mag

            ctx.load512(&VX[i as usize +1], ZRow(60));
            ctx.load512(&VY[i as usize +1], ZRow(61));
            ctx.load512(&VZ[i as usize +1], ZRow(62));
            ctx.extr_yh(3,3);
            ctx.extr_xh(40,0);
            ctx.extr_xh(41,1);
            ctx.extr_xh(42,2);
            ctx.fma64_vec(60,0,3,N_BODIES as u64 - i - 1);
            ctx.fma64_vec(61,1,3,N_BODIES as u64 - i - 1);
            ctx.fma64_vec(62,2,3,N_BODIES as u64 - i - 1);
            ctx.store512(&mut VX[i as usize+1],ZRow(60));
            ctx.store512(&mut VY[i as usize+1],ZRow(61));
            ctx.store512(&mut VZ[i as usize+1],ZRow(62));

            X[i as usize] += dt[0] * VX[i as usize];
            Y[i as usize] += dt[0] * VY[i as usize];
            Z[i as usize] += dt[0] * VZ[i as usize];
        }
    }
}
static ZERO_POINT_FIVE:[f64;8]=[0.5;8];
static ZERO:[f64;8]=[0.0;8];
unsafe fn energy() -> f64 {
    let mut e = 0.0;
    let mut ctx = amx::AmxCtx::new().unwrap();
    ctx.load512(&VX,XRow(0));
    ctx.load512(&VY,XRow(1));
    ctx.load512(&VZ,XRow(2));
    ctx.load512(&MASS,YRow(3));
    ctx.extr_yx(0,0);
    ctx.extr_yx(1,1);
    ctx.extr_yx(2,2);
    ctx.fma64_vec_xy(0,0,0,0);
    ctx.fma64_vec_xy(1,1,1,0);
    ctx.fma64_vec_xy(2,2,2,0);
    ctx.extr_xh(1,0);
    ctx.extr_yh(2,0);
    ctx.fma64_vec_xz(0,0);
    ctx.fma64_vec_yz(0,0);
    ctx.load512(&ZERO_POINT_FIVE,YRow(1));
    ctx.extr_xh(0,0);
    ctx.fma64_vec_xy(0,0,3,0);
    ctx.extr_xh(0,0);
    ctx.fma64_vec_xy(0,0,1,0);
    let mut result = [0.0;8];
    ctx.store512(&mut result,ZRow(0));
    e += result.iter().sum::<f64>();

    let mut tmp_x = [0.0;8];
    let mut tmp_y = [0.0;8];
    let mut tmp_z = [0.0;8];
    let mut tmp_mass = [0.0;8];
    for i in 0..N_BODIES as u64-1 {
      for j in 0..8{
        tmp_x[j] = X[i as usize];
        tmp_y[j] = Y[i as usize];
        tmp_z[j] = Z[i as usize];
        tmp_mass[j] = MASS[i as usize];
      }
      ctx.load512(&tmp_x,ZRow(0));
      ctx.load512(&tmp_y,ZRow(1));
      ctx.load512(&tmp_z,ZRow(2));
      ctx.load512(&X[i as usize +1],XRow(0));
      ctx.load512(&Y[i as usize +1],XRow(1));
      ctx.load512(&Z[i as usize +1],XRow(2));
      ctx.fms64_vec_xz(0,0);
      ctx.fms64_vec_xz(1,1);
      ctx.fms64_vec_xz(2,2);
      ctx.load512(&tmp_mass,YRow(3));
      ctx.load512(&MASS[i as usize +1],XRow(3));
      ctx.fma64_vec_xy(3,3,3,0);
      ctx.extr_xh(0,0);
      ctx.extr_yh(0,0);
      ctx.fma64_vec_xy(0,0,0,0);
      ctx.extr_xh(1,1);
      ctx.extr_yh(1,1);
      ctx.fma64_vec_xy(1,1,1,0);
      ctx.extr_xh(2,2);
      ctx.extr_yh(2,2);
      ctx.fma64_vec_xy(2,2,2,0);
      ctx.extr_xh(1,0);
      ctx.extr_yh(2,0);
      ctx.fma64_vec_xz(0,0);
      ctx.fma64_vec_yz(0,0);
      ctx.extr_xh(0,7);
      ctx.fma64_vec_x(50,7);
      ctx.sqrt(50,51);
      ctx.rcp(51,50);
      ctx.extr_xh(50,0);
      ctx.extr_yh(3,0);
      ctx.load512(&ZERO,ZRow(4));
      ctx.fma64_vec_xy(4,0,0,N_BODIES as u64 -i-1);
      ctx.store512(&mut result,ZRow(4));
      e -= result.iter().sum::<f64>();
    }
    e
}

unsafe fn offset_momentum() {
    let mut ctx = amx::AmxCtx::new().unwrap();

    let mut px = [0.0;8];
    let mut py = [0.0;8];
    let mut pz = [0.0;8];
    ctx.load512(&VX,XRow(1));
    ctx.load512(&VY,XRow(2));
    ctx.load512(&VZ,XRow(3));
    ctx.load512(&MASS,YRow(1));
    ctx.fma64_vec_xy(1,1,1,0);
    ctx.fma64_vec_xy(2,2,1,0);
    ctx.fma64_vec_xy(3,3,1,0);
    ctx.store512(&mut px,ZRow(1));
    ctx.store512(&mut py,ZRow(2));
    ctx.store512(&mut pz,ZRow(3));
    let px:f64 = px.iter().sum();
    let py:f64 = py.iter().sum();
    let pz:f64 = pz.iter().sum();
    VX[0] = - px / SOLAR_MASS;
    VY[0] = - py / SOLAR_MASS;
    VZ[0] = - pz / SOLAR_MASS;
}

fn main() {
unsafe {
    let n = match std::env::args().nth(1) {
            Some(arg) => arg.parse().unwrap_or(1000),
            None => 1000
            };
    let tm = init_time();
    offset_momentum();
    println!("{} {} {} {}",VX[0],VY[0],VZ[0], MASS[0]);
    println!("{:.9}", energy());

    advance(0.01, n);

    println!("{:.9}", energy());
    println!("took {}",time_me(tm));
  }
}
