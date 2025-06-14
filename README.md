## Overview
This project implements a grid-based strategy game where two players compete to occupy as much space as possible. Each player strategically places pieces on the grid to maximize their area of ​​control while blocking their opponent's moves. The goal is to occupy as many squares as possible while minimizing their opponent's options, using an intelligent placement algorithm.
# Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

`./m1_game_engine -f maps/map01 -p1 m1_robots/bender -p2 m1_robots/terminator > game_output.txt`

`Host:
mkdir logs
docker run -v "$(pwd)/solution":/filler/solution -v "$(pwd)/logs":/filler/logs -it filler
Inside container:
./m1_game_engine -f maps/map01 -p1 m1_robots/bender -p2 m1_robots/terminator > logs/bender_vs_terminator.txt 
 `

`./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator > logs/bender_vs_terminator.txt`
## How to run
docker build -t filler .
docker run -v "$(pwd)" -it filler
./linux_game_engine -f maps/map00 -p1 solution/target/debug/filler -p2 linux_robots/bender
./linux_game_engine -f maps/map00 -p1 linux_robots/bender -p2 solution/target/debug/filler
