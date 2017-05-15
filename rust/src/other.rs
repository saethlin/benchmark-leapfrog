extern crate rand;
use rand::distributions::IndependentSample;

#[repr(C)]
struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
    mass: f64,
}

#[repr(C)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let mut time: f64 = 0.;
    let time_step: f64 = 0.08;
    let half_time_step: f64 = time_step/2.;
    let time_limit: f64 = 36.525;

    let normal = rand::distributions::Normal::new(0.0, 0.03);

    let mut particles = Vec::new();
    for i in 0..1000 {
        let p = Particle {
            position: Point {
                x: normal.ind_sample(&mut rand::thread_rng()),
                y: normal.ind_sample(&mut rand::thread_rng()),
                z: normal.ind_sample(&mut rand::thread_rng()),
            },
            velocity: Point {x: 0.0, y: 0.0, z: 0.0},
            acceleration: Point {x: 0.0, y: 0.0, z: 0.0},
            mass: 1e-6
        };
        particles.push(p);
    }

    while time < time_limit {
        integrator_leapfrog_part1(&mut particles, half_time_step);
        time += half_time_step;
        gravity_calculate_acceleration(&mut particles);
        integrator_leapfrog_part2(&mut particles, time_step, half_time_step);
        time += half_time_step;
    }
    println!("{:?}", particles[0].position.x);
}

fn integrator_leapfrog_part1(particles: &mut Vec<Particle>, half_time_step: f64) {
    for ref mut particle in particles {
        particle.position.x += particle.velocity.x * half_time_step;
        particle.position.y += particle.velocity.y * half_time_step;
        particle.position.z += particle.velocity.z * half_time_step;
    }
}

fn integrator_leapfrog_part2(particles: &mut Vec<Particle>, time_step: f64, half_time_step: f64) {
    for ref mut particle in particles {
        particle.velocity.x += particle.acceleration.x * time_step;
        particle.velocity.y += particle.acceleration.y * time_step;
        particle.velocity.z += particle.acceleration.z * time_step;

        particle.position.x += particle.velocity.x * half_time_step;
        particle.position.y += particle.velocity.y * half_time_step;
        particle.position.z += particle.velocity.z * half_time_step;
    }
}

fn gravity_calculate_acceleration(particles: &mut Vec<Particle>) {
    let g = 6.6742367e-11; // m^3.kg^-1.s^-2

    for i in 0..particles.len() {
        let mut acceleration = Point {x: 0.0, y: 0.0, z: 0.0};
        for j in 0..particles.len() {
            if i == j {
                continue;
            }
            let dx = particles[i].position.x - particles[j].position.x;
            let dy = particles[i].position.y - particles[j].position.y;
            let dz = particles[i].position.z - particles[j].position.z;
            let r = (dx*dx + dy*dy + dz*dz).sqrt();
            let prefact = -g/(r*r*r) * particles[j].mass;

            acceleration.x += prefact * dx;
            acceleration.y += prefact * dy;
            acceleration.z += prefact * dz;
        }
        particles[i].acceleration = acceleration;
    }
}
