---
title: Protocol
permalink: /protocol/
nav_order: 1
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


### Flow of a Single Drone

All drones have a pre-determined protocol to follow during the simulation. Figure 1 displays how a drone reacts to different events. While these events are happening, each drone will still follow a set path, try to connect to a beacon and try to establish a connection with a drone or a central hub.

<img src="/IM-WANTEDD/images/drone_flow_chart.png" alt="drone flow chart" />

Figure 1: Flow chart of the protocol that each drone follows.



#### **Flow for RTS/CTS in Half Duplex**
A system sending packets through wireless connections could easily loose information from collisions in messages. Using RTS/CTS reduces the chances of this happening substantially by asking the connection if it is clear to send data. Figure 2 visualizes this flow. 

<img src="/IM-WANTEDD/images/rts_cts_flow_chart.png" alt="rts cts flow chart"/>

Figure 2: Flow chart for sending a message through a half-duplex connection.

### Drone Storage
Each drone has a table of beacon pings. Each row contains the IP address of the drone, beacon identifier, the coordinates of the drone and calculated distance about a ping result. Each time a drone successfully pings a beacon, a new row is added.


### Data Between Nodes
Drones have three nodes it can to communicate with. Nearby drones, the central hub, and beacons. They function on their own or while communicating with all three at the same time.

#### **Nearby Drones**
The network of drones closely resemble a VANET. They work synchronously in a fast pased environment. Following a similar path, they can stay in close proximity and be connected to each other. As explained in figure 1, drones use RTS/CTS before sending a packet. This ensures that communicating over half-duplex does not result in lost packets.

As the network is decentralized, drones do not need to send packets directed towards a specific drone. They can instead use flooding to broadcast the message through the mesh. All connected nodes receives the message.

Nearby drones communicate using DSRC technology. This solution provides a reliable connection between drones on a short to medium range with high data rates. To avoid disturbance, noise and information collision, authentication secures that validated messages are between trusted nodes. 


#### **Central Hub**

The central hub represents a starting and ending point for the drones. This is the node that triangulates the position of a beacon by with the data from the drones. Each time a drone connect with it, the drones' ping results are sent. The connection between the drones and the central hub are similar. Several connections to the drones are possible, but as the central hub does not need to send messages to the drones, they do not need to use RTS/CTS.

#### **Beacons**
Helping in the emergencies require the drones to actively search for beacons. The beacons can be buried deep under the ground after a landslide or avalance. Connecting to the beacon needs to be reliable. 

Avalanche beacons' low frequency provide a long range and reliable connections through the ground which can also last over a long period of time. Additionaly, it is already widely used in the field of search and rescue. Using this technology is therefore be beneficial for the drones in these emergency situations.

### Triangulation

As drones only search and does not rescue, the drones do not need to triangulate the position of a beacon. This task can be given to the central hub when it retrieves the table of ping results. With a table of combined ping result, the central hub can calculate the position of beacons.


### Ad Hoc Connections



### Self Healing



### Considering Errors

