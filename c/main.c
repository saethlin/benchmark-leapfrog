#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "config.c"

typedef struct {
    double x;
    double y;
    double z;
} Point;

typedef struct {
    int size;
    Point* position;
    Point* velocity;
    Point* acceleration;
    double* mass;
} ParticleVector;

int main(void) {
	double G = 6.6742367e-11;

    configuration config = read_config("../config.ini");

    double time_step = config.time_step;
    double half_time_step = time_step/2.0;

    ParticleVector particles;
    particles.size = config.num_particles;
    particles.position = (Point*)calloc(particles.size*10, sizeof(double));
    particles.velocity = particles.position + particles.size;
    particles.acceleration = particles.velocity + particles.size;
    particles.mass = (double*)(particles.acceleration + particles.size);

    for (int i = 0; i < particles.size; i++) {
        particles.mass[i] = 1e-6;
        particles.position[i].x = (double)rand()/(RAND_MAX/0.03);
        particles.position[i].y = (double)rand()/(RAND_MAX/0.03);
        particles.position[i].z = (double)rand()/(RAND_MAX/0.03);
    }

    double time = 0.0;
    while(time <= config.time_limit) {
		for (int i = 0; i < particles.size; i++) {
			particles.position[i].x  += half_time_step * particles.velocity[i].x;
			particles.position[i].y  += half_time_step * particles.velocity[i].y;
			particles.position[i].z  += half_time_step * particles.velocity[i].z;
		}
	
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

		for (int i = 0; i < particles.size; i++) {
			particles.velocity[i].x += time_step * particles.acceleration[i].x;
			particles.velocity[i].y += time_step * particles.acceleration[i].y;
			particles.velocity[i].z += time_step * particles.acceleration[i].z;

			particles.position[i].x += half_time_step * particles.velocity[i].x;
			particles.position[i].y += half_time_step * particles.velocity[i].y;
			particles.position[i].z += half_time_step * particles.velocity[i].z;
		}

		time += time_step;
    }
    printf("%f", particles.position[0].x);
    free(particles.position);
}

