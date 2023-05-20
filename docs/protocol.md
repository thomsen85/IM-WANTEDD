---
title: Protocol
permalink: /protocol/
nav_order: 2
---

# Protocol
<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>


### Drone Storage
Each drone has a table of beacon pings. Each row contains the IP address of the drone, beacon identifier, the coordinates of the drone and calculated distance about a ping result. Each time a drone successfully pings a beacon, a new row is added.

### Flow of a Single Drone

All drones have a pre-determined protocol to follow during the simulation. Figure 1 displays how a drone reacts to different events. While these events are happening, each drone will still follow a set path, try to connect to a beacon and try to establish a connection with a drone or a central hub.

<img src="/IM-WANTEDD/images/drone_flow_chart.png" alt="drone flow chart" />

Figure 1: Flow chart of the protocol that each drone follows.



#### **Flow for RTS/CTS in Half Duplex**
A system sending packets through wireless connections could easily loose information from collisions in messages. Using RTS/CTS reduces the chances of this happening substantially by asking the connection if it is clear to send data. Figure 2 visualizes this flow. 

<img src="/IM-WANTEDD/images/rts_cts_flow_chart.png" alt="rts cts flow chart"/>

Figure 2: Flow chart for sending a message through a half-duplex connection.


### Data Between Nodes
Drones have three nodes it can to communicate with. Nearby drones, the central hub, and beacons. They function on their own or while communicating with all three at the same time.

#### **Nearby Drones**
The network of drones closely resemble a VANET. They work synchronously in a fast pased environment. Following a similar path, they can stay in close proximity and be connected to each other. As explained in figure 1, drones use RTS/CTS before sending a packet. This ensures that communicating over half-duplex does not result in lost packets by collisions. There is overhead using RTS/CTS instead of TDMA, but at the same time secure in all situations where TDMA would require synchronization between nodes to avoid collisions.

As the network is decentralized, drones do not need to send packets directed towards a specific drone. They can instead use flooding to broadcast the message through the mesh. All connected nodes receive the message. If a drone receives a ping result from a another drone, it first checks if it already has it in its table. If it is, it does not send the message further. Otherwise, the message is sent to all conncted drones except for the drones that sent it. A hop count is standard to use with the flooding techique. As the mesh can constantly change and drones need to broadcast the message to all nodes, no hop count is necessary.

Nearby drones communicate using DSRC technology. This solution provides a reliable connection between drones on a short to medium range with high data rates. To avoid disturbance, noise and information collision, authentication secures that validated messages are between trusted nodes.

Connecting drones use TCP for reliable connections. As it is important for the mesh to have the correct information, packet loss tracking in TCP is used.

#### **Central Hub**

The central hub represents a starting and ending point for the drones. This is the node that triangulates the position of a beacon by with the data from the drones. Each time a drone connect with it, the drones' ping results are sent. The connection between the drones and the central hub are similar. Several connections to the drones are possible, but as the central hub does not need to send messages to the drones, they do not need to use RTS/CTS.

#### **Beacons**
Helping in the emergencies require the drones to actively search for beacons. The beacons can be buried deep under the ground after a landslide or avalance. Connecting to the beacon needs to be reliable. 

Avalanche beacons' low frequency provide a long range and reliable connections through the ground which can also last over a long period of time. Additionaly, it is already widely used in the field of search and rescue. Using this technology is therefore be beneficial for the drones in these emergency situations.

### Triangulation

As drones only search and does not rescue, the drones do not need to triangulate the position of a beacon. This task can be given to the central hub when it retrieves the table of ping results. With a table of combined ping result, the central hub can calculate the position of beacons.


### Ad-Hoc Connections

A requirement for ad-hoc mesh networks is to be able to manage new connections. As the network is decentralized, making sure all drones have the available and most updated information can be tricky. 

Drones always look for the central hub or other drones it can connect to. When a new drone is in range, it is connected by a TCP three-way handshake.

To make the list of ping results between drones synchronized, on of the drones in the connection sends the entire ping result table. This list of ping results are cross checked with its own list. Ping results that it already has are ignored, new rows are sent through the network except for the source drone and rows that it has which were not sent by the source drone are sent to the source. This ensures synchronization between the meshes that the drones were connected to.

### Self Healing
All drones always try to connect to drones in the area if they are in range. Lost connections to drones that are lost are in that way always trying to reconnect. The emergency situation that the drones are in, require them to handle lost connections and still be able to provide the best information to the central hub. Keeping a mesh that is as fully connected as possible is important for synchronizing new ping results.

Another improvement of that this provides is that packets sent between nodes only need to contain one single ping result and not the entire table which is sent on new connections. This reduces the overhead of sending packets between nodes. Keeping connections are therefore wanted.


[Previous: Theory](/IM-WANTEDD/theory){: .btn .btn-blue }
[Next: Simulation](/IM-WANTEDD/simulation){: .btn .btn-blue }