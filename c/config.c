#include <stdio.h>
#include <stdlib.h>

typedef struct {
    double time_limit, time_step;
    int num_particles;
} configuration;

configuration read_config(char* path) {
    configuration config;
    config.time_limit = 3.6525;
    config.time_step = 0.08;
    config.num_particles = 1000;
    return config;
}
