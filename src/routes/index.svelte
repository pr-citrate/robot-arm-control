<!-- src/routes/index.svelte -->

<script lang="ts">
    import { onMount } from 'svelte';
    import { robotState } from '$lib/stores/robotStore';
    import { invoke } from '@tauri-apps/api/tauri';
    import { writable } from 'svelte/store';

    let localState = {
        J1: 90,
        J2: 90,
        J3: 90,
        J4: 90,
        J5: 90,
        J6: 90,
        Di1: false,
        Di2: false,
        Di3: false,
        Do1: false,
        Do2: false,
        Do3: false,
        robotSpeed: 50,
    };

    const updateRobot = async () => {
        try {
            await invoke('send_robot_commands', { robotState: localState });
        } catch (error) {
            console.error('Error sending robot commands:', error);
        }
    };

    const fetchRobotState = async () => {
        try {
            const state = await invoke('read_robot_state');
            robotState.set(state);
            localState = { ...state };
        } catch (error) {
            console.error('Error reading robot state:', error);
        }
    };

    // Initialize serial port on mount
    onMount(async () => {
        try {
            await invoke('initialize_serial', { port: '/dev/ttyUSB0', baud_rate: 9600 });
            console.log('Serial port initialized');
        } catch (error) {
            console.error('Failed to initialize serial port:', error);
        }

        // Fetch robot state periodically
        setInterval(fetchRobotState, 1000);
    });
</script>

<div class="min-h-screen bg-sky-light text-slate-800 p-8">
    <h1 class="text-4xl font-bold mb-6 text-sky-dark">6-Axis Robot Arm Controller</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <!-- Servo Controls -->
        <div class="bg-white shadow-md rounded-lg p-6">
            <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Servo Controls</h2>
            {#each ['J1', 'J2', 'J3', 'J4', 'J5', 'J6'] as servo}
                <div class="mb-4">
                    <label class="block text-sky-dark mb-2" for={servo}>
                        {servo}: {localState[servo]}
                    </label>
                    <input
                        type="range"
                        id={servo}
                        min="0"
                        max="180"
                        bind:value={localState[servo]}
                        class="w-full"
                        on:input={updateRobot}
                    />
                </div>
            {/each}
            <div class="mb-4">
                <label class="block text-sky-dark mb-2" for="robotSpeed">
                    Robot Speed: {localState.robotSpeed}
                </label>
                <input
                    type="range"
                    id="robotSpeed"
                    min="0"
                    max="100"
                    bind:value={localState.robotSpeed}
                    class="w-full"
                    on:input={updateRobot}
                />
            </div>
        </div>

        <!-- Digital Outputs -->
        <div class="bg-white shadow-md rounded-lg p-6">
            <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Digital Outputs</h2>
            {#each ['Do1', 'Do2', 'Do3'] as doPin}
                <div class="flex items-center mb-4">
                    <input
                        type="checkbox"
                        id={doPin}
                        bind:checked={localState[doPin]}
                        class="mr-2"
                        on:change={updateRobot}
                    />
                    <label class="text-sky-dark" for={doPin}>{doPin}</label>
                </div>
            {/each}
        </div>
    </div>

    <!-- Digital Inputs -->
    <div class="mt-8 bg-white shadow-md rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Digital Inputs</h2>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
            {#each ['Di1', 'Di2', 'Di3'] as diPin}
                <div class="flex items-center">
                    <span class="text-sky-dark mr-2">{diPin}:</span>
                    <span
                        class={`px-2 py-1 rounded ${
                            $robotState[diPin] ? 'bg-green-500' : 'bg-red-500'
                        } text-white`}
                    >
                        {$robotState[diPin] ? 'ON' : 'OFF'}
                    </span>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    /* Add any additional styles if necessary */
</style>
