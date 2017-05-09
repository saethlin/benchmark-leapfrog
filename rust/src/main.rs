// Originally 1.24 seconds
// With Particle and Point struct is 1.38 seconds

extern crate rand;
use rand::distributions::IndependentSample;

#[repr(C)]
struct ParticleVector {
    data: Vec<f64>,
    size: usize,
}

#[repr(C)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn as_point_slice_mut(slice: &mut [f64]) -> &mut [Point] {
    let p = slice.as_mut_ptr();
    let n = slice.len();
    assert!(n%3 == 0);
    unsafe {
        std::slice::from_raw_parts_mut(p as *mut Point, n/3)
    }
}

fn as_point_slice(slice: &[f64]) -> &[Point] {
    let p = slice.as_ptr();
    let n = slice.len();
    assert!(n%3 == 0);
    unsafe {
        std::slice::from_raw_parts(p as *const Point, n/3)
    }
}

impl ParticleVector {
    fn new(size: usize) -> Self {
        ParticleVector {
            data: vec![0.0; size*10], // 3 for each of position, velocity, and acceleration and 1 for mass
            size: size,
        }
    }
    fn position_mut(&mut self) -> &mut [Point] {
        as_point_slice_mut(&mut self.data[0..self.size*3])
    }
    fn velocity_mut(&mut self) -> &mut [Point] {
        as_point_slice_mut(&mut self.data[self.size*3..self.size*6])
    }
    fn acceleration_mut(&mut self) -> &mut [Point] {
        as_point_slice_mut(&mut self.data[self.size*6..self.size*9])
    }
    fn mass_mut(&mut self) -> &mut[f64] {
        &mut self.data[self.size*9..self.size*10]
    }
    fn mass(&self) -> &[f64] {
        &self.data[self.size*9..self.size*10]
    }
    fn position(&self) -> &[Point] {
        as_point_slice(&self.data[0..self.size*3])
    }
    fn velocity(&self) -> &[Point] {
        as_point_slice(&self.data[self.size*3..self.size*6])
    }
    fn acceleration(&self) -> &[Point] {
        as_point_slice(&self.data[self.size*6..self.size*9])
    }
}


fn main() {
    let mut time: f64 = 0.;
    let time_step: f64 = 0.08;
    let half_time_step: f64 = time_step/2.;
    let time_limit: f64 = 365.25;

    let normal = rand::distributions::Normal::new(0.0, 0.03);

    let mut particles = ParticleVector::new(100);
    for i in 0..particles.size {
        particles.mass_mut()[i] = 1e-6;
        particles.position_mut()[i] = Point {
            x: normal.ind_sample(&mut rand::thread_rng()),
            y: normal.ind_sample(&mut rand::thread_rng()),
            z: normal.ind_sample(&mut rand::thread_rng())
        }
    }

    while time < time_limit {
        integrator_leapfrog_part1(&mut particles, half_time_step);
        time += half_time_step;
        gravity_calculate_acceleration(&mut particles);
        integrator_leapfrog_part2(&mut particles, time_step, half_time_step);
        time += half_time_step;
    }
    println!("{:?}", particles.position()[0].x);
}

fn integrator_leapfrog_part1(particles: &mut ParticleVector, half_time_step: f64) {
    for i in 0..particles.size {
        particles.position_mut()[i].x += particles.velocity()[i].x * half_time_step;
        particles.position_mut()[i].y += particles.velocity()[i].y * half_time_step;
        particles.position_mut()[i].z += particles.velocity()[i].z * half_time_step;
    }
}

fn integrator_leapfrog_part2(particles: &mut ParticleVector, time_step: f64, half_time_step: f64) {
    for i in 0..particles.size {
        particles.velocity_mut()[i].x += particles.acceleration()[i].x * time_step;
        particles.velocity_mut()[i].y += particles.acceleration()[i].y * time_step;
        particles.velocity_mut()[i].z += particles.acceleration()[i].z * time_step;

        particles.position_mut()[i].x += particles.velocity()[i].x * half_time_step;
        particles.position_mut()[i].y += particles.velocity()[i].y * half_time_step;
        particles.position_mut()[i].z += particles.velocity()[i].z * half_time_step;
    }
}

fn gravity_calculate_acceleration(particles: &mut ParticleVector) {
    let g = 6.6742367e-11; // m^3.kg^-1.s^-2

    for i in 0..particles.size {
        let mut acceleration = Point {x: 0.0, y: 0.0, z: 0.0};
        for j in 0..particles.size {
            if i == j {
                continue;
            }
            let dx = particles.position()[i].x - particles.position()[j].x;
            let dy = particles.position()[i].y - particles.position()[j].y;
            let dz = particles.position()[i].z - particles.position()[j].z;
            let r = (dx*dx + dy*dy + dz*dz).sqrt();
            let prefact = -g/(r*r*r) * particles.mass()[j];

            acceleration.x += prefact * dx;
            acceleration.y += prefact * dy;
            acceleration.z += prefact * dz;
        }
        particles.acceleration_mut()[i] = acceleration;
    }
}
