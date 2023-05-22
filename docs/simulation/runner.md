---
title: Simulation runnner
permalink: /simulation-runner/
parent: Simulation
nav_order: 2
layout: minimal
---

# This is the simulation runner
Loading in the simulation might take some time. If you want to reset the simulation, you can refresh the page. Be aware that switching tabs while the simulation is running might cause unexpected changes in the topology.


<script type="module">
    import init from '/IM-WANTEDD/simulation/simulation_runner/simulation.js';

    async function run() {
        await init();
    }
    run();
</script>