---
title: Simulation
permalink: /simulation/
nav_order: 3
has_children: true
---

# Simulation

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

<span class="fs-6">
[Run the simulation](/IM-WANTEDD/simulation-runner){: .btn .btn-green  }
</span>


## Landscape

Running the simulation shows drones moving through a landscape. The landscape chosen is Gjerdrum. In decemeber 2020, a large landslide occured in Gjerdrum. 10 people died where the last was found in march 2021. The area to search was very large, and terrain that was difficult and dangerous to traverse (Wikipedia, 2023). Drones would in this case be useful to systematically search for victims effeciently. The operation would not rely on people to traverse the terrain. The drones would with an already existing beacon technology, be able to locate victims accurately.

Arial photos of the landscape after 2020 has been gathered and combined into a single photo. This photo has then been used with a DSM of the area to create a 3D model of the landscape in [QGIS](https://qgis.org/en/site/). The DSM is downloaded from [Kartverket høydedata](https://hoydedata.no/) Figure 3 shows the terrain photo used in the simulation. Unfortunately, the 3D visualization has a capacity for the detail it can use from the DSM so the terrain is not as detailed as it could be, but it still visualizes the landscape's features.

<img src="/IM-WANTEDD/images/gjerdrum.png" alt="Arial photo of Gjerdrum" />
Figure 3: Arial photo of Gjerdrum

## Modifying the simulation

The simulation starts by loading in the simulation canvas. This might take a few seconds depending on you internet connection. When it is loaded in, it offers the user to choose between different scenarios. They modify how many drones are present with their position, beacons to triangulate and the connection between drones. When a scenario is selected, the drones load in and starts moving. 

By just using a mouse or a touchpad, it is possible to change the camera's angle and position.

During the simulation, you can interact with it using the user interface controls. There are three UI components that the user can interact with. The first one is the controls which modifies the simulation. 

Changes that can be made includes:
- Camera angle and position
- Connection distance between drones
- Displaying the drone's ID
- Displaying the drone's beacon ping radii
- The height of the terrain to reveal or hide beacon

The second is a list of the drones with a button for their details. This button opens a third window. This shows a table of their ping results. This table includes the columns for the drone's ID, the beacon's ID, the distance between them, the time the ping result was made and the coordinates of the drone when the ping result was made.


## Abstractions

As the simulation is a visual representation of the protocol, there are parts of the protocol that has been abstracted. Still, the simulation code tries to be similar to the protocol implementation. The simulation shows mostly the physical layer of the OSI model. This includes showing when the connection is made between drones and beacons and the packets that are sent between drones. 

The information that is sent between them is shown in the drone's detail page in the form of its ping result storage. The headers and the payload in the packets are abstracted from the end user. This is despite that they are sent between the objects in the simulation code.

Another abstraction is how drones create a connection between them. In the simulation, drones connect when they get in range of each other. In reality this connection would instead happen by the TCP three-way handshake as decsribe in the protocol section.

The simulation shows that packages are not sent in both directions at the same time. This is because of half-duplex communication. What information is sent in the connection with RTS/CTS, is abstracted to the end user.

The distance that are in the ping results would be derived from the time it took to receive the ping result. To help visualize distance to beacons, the distance is given directly to the drone in the simulation. With a central hub, the computation would happen there instead.


<br/>
[Previous: Protocol](/IM-WANTEDD/protocol){: .btn .btn-blue }
[Next: Further work](/IM-WANTEDD/further-work){: .btn .btn-blue }
