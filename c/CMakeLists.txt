cmake_minimum_required(VERSION 3.6)
project(benchmark-leapfrog-c)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_COMPILER /usr/bin/clang++)

set(SOURCE_FILES main.c )

link_libraries(m)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -Wall -Wextra -march=native -O3 -static -flto")

add_executable(main ${SOURCE_FILES})