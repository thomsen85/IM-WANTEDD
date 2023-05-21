---
title: Theory
permalink: /theory/
nav_order: 1
---

## Theory

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

## Mesh networks

Mesh networks describe a connection of nodes which can communicate with each other directly without relying on a central node. They can form different topologies and can be either partial or full. Full meshes forms a complete graph where all nodes are connected to each other. When using meshes in the open space, interference can cause unwanted errors in connectivity. More connections to secure packets are received might be beneficial (Young, 2008).

### Flooding
A technique used in mesh network is flooding. By broadcasting a packet to all connected nodes which in turn also transmits to its node except for source. The message is thus transmitted. To avoid overhead while also only targeting a set of nodes, a hop count can be used to only send a packet a defined amount of layers from the initial source node (Subramanian, 2012).

## Ad-hoc networks

Ad-hoc networks is a solution for networks to react to new connections in a network. Nodes in this network can be between computers and routers (CompTIA, 2019).

## Vanet and Fanets

Manet is a network of mobile nodes which can connect and disconnect at will and Vanet is the subset that is specilized for vehicles. Vanets provides high node mobility. Fanet is a subset of Vanet used for flying vehicles like planes and UAVs. These networks can be used in a combination of several different network topologies like centralized, decentralized, multigroup or multilayer. (Suescun, 2016).

## Half Duplex

In wireless networks, data can be sent both ways, but only one way at a time. This is called half duplex. It is in contrast of full duplex used in wired transmission where data can be sent both ways at the same time (Burke, 2019). To avoid collisions, there needs to be a check between nodes to see if they are ready to receive. This is dependent on the network topolgy.

## RTS/CTS

The protocol for RTS/CTS can be used in half duplex connections primarily for hidden-node problem. It avoids collsions in packets by first sending a request to send a message. If it receives a clear to send message, it can send the message. If it does not receive a clear to send message, it will wait a random amount of time before sending a new request to send message. The time delay before sending a new request needs to be random to avoid creating a collision with another node's request continuously (Olsø, 2017). This method is in certain situations in contrast to TDMA which is a method for dividing a channel into time slots. This provides some challenges where two separate meshes meet. Synchronization between the meshes is critical in these cases (Hadded, 2015).


## DSRC

DSRC is a vehicle to vehicle and vehicle to infrastructure protocol used in Vanet. With datarates in the megabits and a range of up to 1000 meters, it helps vehicles send updated information about traffic and emergencies. There are differences between countries, but the standards in the different regions follow the same principal. DSRC uses a frequency around 5.9GHz with a bandwith of tens of megahertz depending on the region. A small bandwith helps to exclude noice which might generate from the waves' interaction with the environment (Hadded, 2015).

To avoid anyone intercept this network, HMAC can be used to authenticate messages with performance which will affect the networks delay's minimally. The delay, security and delivery ratio is dependent on the hash key size (Khare, 2020).

## Avalanche Beacons
Avalanche beacons are used to connect two beacons. On beacon is in search mode which searches for the beacon in receive mode. A frequency of 457 KHz is used for reasons like reliable and distant connections through any kind of material. This is neccessary when a beacon is buried deep under snow. Each beacon can be identified through the position of the signal pulse edge on the time axis (Meier, 2008).


<br/>
[Previous: Intro](/IM-WANTEDD/){: .btn .btn-blue }
[Next: Protocol](/IM-WANTEDD/protocol){: .btn .btn-blue }