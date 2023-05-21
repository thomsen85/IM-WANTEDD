---
title: Further work
permalink: /further-work/
nav_order: 4
---

# Further work

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

## Central hub

In the simulation we have decided to not include the central hub. This was not prioritized because the connection the drones have with each other is very similar to that of the central hub.

An implementation that we would want to see in the future, is a routing protocol used to communicate with the central hub. As the entire mesh network approaches the central hub, the routing protocol would make only one drone connect to it. As the mesh is always up to date, only one drone would need to send its stored information. We could in this protocol use OLSR to see if any other drones in the network are connected. If so, no new connections would need to be made. 

As the simulation does not include the central hub, there is no need to show the calculation of the triangulation. The central hub could also be the point that executes the computation of the distance between the drones and beacons, but to better visualize, the distance is given in the simulation.


## Considering inaccurate data

In reality, drones might have software or hardware bugs that product data which is wrong. To partially solve this, comparing data amongst drones would help find what data is most likely correct and if there are any outliars. This could for example be done by looking at the data for one drone by seeing if the distance to the beacon corresponds with its other ping results relative to the timestamp. An extension of this would be to compare results accross drones and use the distance between them to calculate if the found distances to the beacons are within reasonable range.

<br/>
[Previous: Simulation](/IM-WANTEDD/simulation){: .btn .btn-blue }
[Next: Bibliography](/IM-WANTEDD/bibliography){: .btn .btn-blue }
