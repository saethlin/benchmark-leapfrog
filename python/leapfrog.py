import numpy as np
import numba
import time


@numba.jit(nopython=True, cache=True)
def leapfrog_slow(position, velocity, acceleration, mass):
    G = 6.67e-11
    time = 0.0
    while time < time_limit:
        # Step 1
        position += half_time_step * velocity

        # Calc acceleration
        acceleration[:, :] = 0.0
        for i in range(num_particles):
            for j in range(num_particles):
                if i == j:
                    continue
                dx = position[i, 0] - position[j, 0]
                dy = position[i, 1] - position[j, 1]
                dz = position[i, 2] - position[j, 2]
                r = np.sqrt(dx**2 + dy**2 + dz**2)
                prefact = -G/(r**3) * mass[j]
                acceleration[i, 0] += prefact * dx
                acceleration[i, 1] += prefact * dy
                acceleration[i, 2] += prefact * dz

        # Step 2
        velocity += time_step * acceleration
        position += half_time_step * velocity

        time += time_step

    return position


@numba.jit(nopython=True, cache=True)
def leapfrog(position, velocity, acceleration, mass):
    G = 6.67e-11
    time = 0.0
    while time < time_limit:
        # Step 1
        for i in range(num_particles):
            position[i, 0] += half_time_step * velocity[i, 0]
            position[i, 1] += half_time_step * velocity[i, 1]
            position[i, 2] += half_time_step * velocity[i, 2]

        time += half_time_step

        # Calc acceleration
        acceleration[:, :] = 0.0
        for i in range(num_particles):
            for j in range(num_particles):
                if i == j:
                    continue
                dx = position[i, 0] - position[j, 0]
                dy = position[i, 1] - position[j, 1]
                dz = position[i, 2] - position[j, 2]
                r = np.sqrt(dx**2 + dy**2 + dz**2)
                prefact = -G/(r**3) * mass[j]
                acceleration[i, 0] += prefact * dx
                acceleration[i, 1] += prefact * dy
                acceleration[i, 2] += prefact * dz

        # Step 2
        for i in range(num_particles):
            velocity[i, 0] += time_step * acceleration[i, 0]
            velocity[i, 1] += time_step * acceleration[i, 1]
            velocity[i, 2] += time_step * acceleration[i, 2]

            position[i, 0] += half_time_step * velocity[i, 0]
            position[i, 1] += half_time_step * velocity[i, 1]
            position[i, 2] += half_time_step * velocity[i, 2]

        time += half_time_step

    return position


time_step = 0.08
half_time_step = time_step/2
time_limit = 365.25
num_particles = 3

position = np.random.normal(0, 0.03, (num_particles, 3))
velocity = np.zeros_like(position)
acceleration = np.zeros_like(position)
mass = np.ones(num_particles) * 1e-6

leapfrog(position, velocity, acceleration, mass)

start = time.time()
position = leapfrog(position, velocity, acceleration, mass)
print(time.time()-start)


from tkinter import *


# User input refine rate
def refineRate(event):
    if refineRate == int():
        refineComp = int(refineRate)
        return refineComp
    elif refineRate == float():
        refineComp = float(refineRate)
        return refineComp


# veldspar calculator
def veldCalc(event):
    minValue = open("mineral_value.csv", "r")
    veld = minValue.readlines()[0]
    minValue.close()
    refine = refineRate(event)
    veld = veld[0:3]
    veld = int(veld)
    veld = veld / 100 * (refine)
    refinedVeld = veld * int(veldCalc)
    print(refinedVeld)


root = Tk()
root.title("Ore Calculator")
root.geometry("600x600")
root.resizable(width=False, height=False)

# Reprocessing efficancy input
repro = Label(root, text="Reprocessing %")
repro_entry = Entry(root)
repro.grid(row=0, column=0)
repro_entry.grid(row=0, column=1)
repro_entry.bind("<KeyPress>", refineRate)

# Veld input
veld = Label(root, text="Veldspar: ")
veld_entry = Entry(root)
veld.grid(row=1, column=0)
veld_entry.grid(row=1, column=1)
veld_entry.bind("<KeyPress>", veldCalc)

root.mainloop()