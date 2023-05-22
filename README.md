# **IM-WANTEDD**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/thomsen85/IM-WANTEDD/actions/workflows/ci.yml/badge.svg)](.github/workflows/ci.yml)
[![CD](https://github.com/thomsen85/IM-WANTEDD/actions/workflows/pages/pages-build-deployment/badge.svg)](https://thomsen85.github.io/IM-WANTEDD)


> A mobile wireless mesh network simulation for rescue missions using drones.

Created by [Thomas Svendal](https://github.com/thomsen85) and [Carl Gützkow](https://github.com/cjgutz)

<u>**[Run the simulation here](https://thomsen85.github.io/IM-WANTEDD/simulation-runner/)**</u>

IM-WANTEDD is a simulation program written in Rust with Bevy, designed to visualize and simulate a mobile wireless mesh network for rescue missions. The simulation includes a network of drones that communicate with each other to triangulate the position of nearby beacons. The program aims to provide an interactive and realistic environment to study and analyze the behavior of the network in realistic scenarios.

## Documentation and report

Visit the [repository report](https://thomsen85.github.io/IM-WANTEDD/) for a guide, documentation and an in depth look at the project.

## Features

- Simulates a mobile wireless mesh network with drones for rescue missions.
- Realistic visualization and simulation of drone movements and communication.
- Dynamic network topology with decentralized communication.
- Easy-to-use interface for controlling and monitoring the simulation.

## External Dependencies (Crates)

- **Bevy**: A game engine built in Rust. [Website](https://bevyengine.org/). Used for creating the simulation graphics.
- **Bevy egui**: A user interface for user controls that works with bevy. [Website](https://github.com/mvlabat/bevy_egui). Used for adding simulation controls.
- **Rand**: A tool for creating random numbers. [Website](https://docs.rs/rand/latest/rand/). Not currently used in this project, but a great tool to have for future development.
- **Chrono**: A date and time library. [Website](https://github.com/chronotope/chrono). Used for timestamps for ping results in the simulation.

## Installation

1. Ensure you have Rust and Cargo installed. Refer to the [Rust documentation](https://www.rust-lang.org/tools/install) for installation instructions.
   
2. Follow [this](https://bevyengine.org/learn/book/getting-started/setup#install-os-dependencies) installation guide in case dependencies are not fulfilled.


3. Clone the IM-WANTEDD repository:

```
git clone git@github.com:thomsen85/IM-WANTEDD.git
```

4. Navigate to the project directory:
```
cd IM-WANTEDD
```

5. Build and run the simulation:
```
cargo run --release
```

## Troubleshooting

If you have any trouble running the simulation, please refer to the Bevy installation guide [here](https://bevyengine.org/learn/book/getting-started/setup#install-os-dependencies). If problems persist, please open an [issue](https://github.com/thomsen85/IM-WANTEDD/issues/new).

## Usage

The simulation starts by the user choosing a scenario. The scenario determines how many drones are used, the connection between them and beacons in the world. When a scenario is selected, the drones load in and starts moving. During the simulation, you can interact with it using the user interface controls. Changes that can be made during the simulation includes:
- Connection distance between drones
- Displaying the drone's ID
- Displaying the drone's beacon ping radii
- The height of the terrain to reveal or hide beacon

It is possible to change the camera's angle and position by dragging the screen

## CI/CD
There is one continous integration and one continous deployment script in the project.
CI is used for building, checking linting and running tests in the project.
You can find the latest run [here](https://github.com/thomsen85/IM-WANTEDD/actions/runs/5036642913). The workflow file is found [here](.github/workflows/ci.yml).
CD is used for building the documentation and deploying it to github pages.
You can find the latest run [here](https://github.com/thomsen85/IM-WANTEDD/actions/runs/5036642845).

## License

IM-WANTEDD is licensed under the [MIT License](LICENSE).
