---
title: Simulation runnner
permalink: /runner/
parent: Simulation
nav_order: 2
---

# This is the simulation runner
Loading in the simulation might take some time


<script type="module">
    import init from '/IM-WANTEDD/simulation/simulation_runner/simulation.js';

    async function run() {
        await init();
    }
    run();
</script>