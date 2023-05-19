---
layout: page
title: Protocol
permalink: /protocol/
---

### Flow of a Single Drone

All drones have a pre-determined protocol to follow during the simulation. Figure 1 displays how a drone reacts to different events. While these events are happening, each drone will still follow a set path, try to connect to a beacon and try to establish a connection with a drone or a central hub.

<img src="/images/drone_flow_chart.png" alt="drone flow chart" />

Figure 1: Flow chart of the protocol that each drone follows.



#### **Flow for RTS/CTS in Half Duplex**
A system sending packets through wireless connections could easily loose information from collisions in messages. Using RTS/CTS reduces the chances of this happening substantially by asking the connection if it is clear to send data. Figure 2 visualizes this flow. 

<img src="/images/rts_cts_flow_chart.png" alt="rts cts flow chart"/>

Figure 2: Flow chart for sending a message through a half-duplex connection.


### Data Between Nodes



### Triangulation



### Considering Errors



### Ad Hoc Connections



### Self Healing



### Central Hub