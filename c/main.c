#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef struct Point {
    double x;
    double y;
    double z;
} Point;

typedef struct ParticleVector {
    double* data;
    int size;
    Point* position;
    Point* velocity;
    Point* acceleration;
    double* mass;
} ParticleVector;

void integrator_leapfrog_part1(ParticleVector particles, double half_time_step){
    for (int i = 0; i < particles.size; i++) {
        particles.position[i].x  += half_time_step * particles.velocity[i].x;
        particles.position[i].y  += half_time_step * particles.velocity[i].y;
        particles.position[i].z  += half_time_step * particles.velocity[i].z;
    }
}

void integrator_leapfrog_part2(ParticleVector particles, double time_step, double half_time_step){
    for (int i = 0; i < particles.size; i++) {
        particles.velocity[i].x += time_step * particles.acceleration[i].x;
        particles.velocity[i].y += time_step * particles.acceleration[i].y;
        particles.velocity[i].z += time_step * particles.acceleration[i].z;

        particles.position[i].x += half_time_step * particles.velocity[i].x;
        particles.position[i].y += half_time_step * particles.velocity[i].y;
        particles.position[i].z += half_time_step * particles.velocity[i].z;
    }
}

void gravity_calculate_acceleration(ParticleVector particles) {
    double G = 6.6742367e-11; // m^3.kg^-1.s^-2
    for (int i = 0; i < particles.size; i++) {
        particles.acceleration[i].x = 0.0;
        particles.acceleration[i].y = 0.0;
        particles.acceleration[i].z = 0.0;

        for (int j = 0; j < particles.size; j++){
            if (j == i) {
                continue;
            }
            double dx = particles.position[i].x - particles.position[j].x;
            double dy = particles.position[i].y - particles.position[j].y;
            double dz = particles.position[i].z - particles.position[j].z;
            double r = sqrt(dx*dx + dy*dy + dz*dz);
            double prefact = -G/(r*r*r) * particles.mass[j];
            particles.acceleration[i].x += prefact * dx;
            particles.acceleration[i].y += prefact * dy;
            particles.acceleration[i].z += prefact * dz;
        }
    }
}

int main(int argc, char* argv[]) {
    const int n_particles = 100;
    double time = 0;
    double time_step = 0.08;
    double half_time_step = time_step/2;
    double time_limit = 365.25;

    ParticleVector particles;
    particles.size = n_particles;
    particles.data = (double*)calloc(n_particles*10, sizeof(double));
    particles.position = (Point*)particles.data;
    particles.velocity = (Point*)(particles.data + particles.size*3);
    particles.acceleration = (Point*)(particles.data + particles.size*6);
    particles.mass = particles.data + particles.size * 9;

    for (int i = 0; i < n_particles; i++) {
        particles.mass[i] = 1e-6;
        particles.position[i].x = (double)rand()/(double)(RAND_MAX/0.03);
        particles.position[i].y = (double)rand()/(double)(RAND_MAX/0.03);
        particles.position[i].z = (double)rand()/(double)(RAND_MAX/0.03);
    }

    while(time <= time_limit) {
        integrator_leapfrog_part1(particles, half_time_step);
        time += half_time_step;
        gravity_calculate_acceleration(particles);
        integrator_leapfrog_part2(particles, time_step, half_time_step);
        time += half_time_step;
    }
    printf("%f", particles.position[0].x);
    free(particles.data);
}

