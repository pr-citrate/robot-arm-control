<!-- src/routes/index.svelte -->

<script lang="ts">
    import { onMount } from 'svelte';
    import { robotState } from '$lib/stores/robotStore';
    import { invoke } from "@tauri-apps/api/core";
    import { writable } from 'svelte/store';

    interface StateInterface {
        J1: number;
        J2: number;
        J3: number;
        J4: number;
        J5: number;
        J6: number;
        Di1: boolean;
        Di2: boolean;
        Di3: boolean;
        Do1: boolean;
        Do2: boolean;
        Do3: boolean;
        robotSpeed: number;
    }

    let localState: StateInterface = {
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

    const getJointState = (state: StateInterface): Record<string, number> => {
        return {
            J1: state.J1,
            J2: state.J2,
            J3: state.J3,
            J4: state.J4,
            J5: state.J5,
            J6: state.J6,
        }
    }

    const getDigitalPinState = (state: StateInterface): Record<string, boolean> => {
        return {
            Di1: state.Di1,
            Di2: state.Di2,
            Di3: state.Di3,
            Do1: state.Do1,
            Do2: state.Do2,
            Do3: state.Do3,
        }
    }

    let jointState: Record<string, number> = getJointState(localState);
    let digitalPinState: Record<string, boolean> = getDigitalPinState(localState);

    // New: Available Serial Ports
    const availablePorts = writable<string[]>([]);
    let selectedPort: string = '';
    let baudRate: number = 9600; // Default baud rate

    // New: Status Messages
    const statusMessage = writable<string>('');

    const updateRobot = async () => {
        try {
            await invoke('send_robot_commands', { robotState: localState });
            statusMessage.set('Commands sent successfully.');
        } catch (error) {
            console.error('Error sending robot commands:', error);
            statusMessage.set('Failed to send commands.');
        }
    };

    const fetchRobotState = async () => {
        try {
            const state: StateInterface = await invoke('read_robot_state');
            robotState.set(state);
            localState = { ...state };
            jointState = getJointState(localState);
            digitalPinState = getDigitalPinState(localState);
        } catch (error) {
            console.error('Error reading robot state:', error);
            statusMessage.set('Failed to read robot state.');
        }
    };

    const listPorts = async () => {
        try {
            const ports: string[] = await invoke('list_serial_ports');
            availablePorts.set(ports);
        } catch (error) {
            console.error('Error listing serial ports:', error);
            statusMessage.set('Failed to list serial ports.');
        }
    };

    const initializePort = async () => {
        if (!selectedPort) {
            statusMessage.set('Please select a serial port.');
            return;
        }

        try {
            await invoke('initialize_serial', { port: selectedPort, baud_rate: baudRate });
            statusMessage.set(`Serial port ${selectedPort} initialized.`);
            // Optionally, fetch robot state immediately after initialization
            await fetchRobotState();
        } catch (error) {
            console.error('Failed to initialize serial port:', error);
            statusMessage.set(`Failed to initialize serial port ${selectedPort}.`);
        }
    };

    // Initialize serial port list on mount
    onMount(async () => {
        await listPorts();

        // Optionally, auto-select the first available port
        availablePorts.subscribe(ports => {
            if (ports.length > 0 && !selectedPort) {
                selectedPort = ports[0];
            }
        });

        // Fetch robot state periodically if serial port is initialized
        setInterval(fetchRobotState, 1000);
    });
</script>

<div class="min-h-screen bg-sky-light text-slate-800 p-8">
    <h1 class="text-4xl font-bold mb-6 text-sky-dark">6-Axis Robot Arm Controller</h1>

    <!-- Serial Port Selection -->
    <div class="mb-8 bg-white shadow-md rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Serial Port Configuration</h2>
        <div class="flex items-center space-x-4">
            <div>
                <label for="serialPort" class="block text-sky-dark mb-2">Serial Port:</label>
                <select
                    id="serialPort"
                    bind:value={selectedPort}
                    class="w-full border border-sky-dark rounded p-2"
                >
                    <option value="" disabled>Select a port</option>
                    {#each $availablePorts as port}
                        <option value={port}>{port}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="baudRate" class="block text-sky-dark mb-2">Baud Rate:</label>
                <input
                    type="number"
                    id="baudRate"
                    bind:value={baudRate}
                    min="300"
                    max="115200"
                    step="300"
                    class="w-full border border-sky-dark rounded p-2"
                />
            </div>
            <div class="flex items-end">
                <button
                    on:click={initializePort}
                    class="bg-sky-dark text-white px-4 py-2 rounded hover:bg-sky-700"
                >
                    Initialize
                </button>
            </div>
        </div>
        {#if $statusMessage}
            <p class="mt-4 text-sm text-red-500">{$statusMessage}</p>
        {/if}
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <!-- Servo Controls -->
        <div class="bg-white shadow-md rounded-lg p-6">
            <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Servo Controls</h2>
            {#each ['J1', 'J2', 'J3', 'J4', 'J5', 'J6'] as servo}
                <div class="mb-4">
                    <label class="block text-sky-dark mb-2" for={servo}>
                        {servo}: {jointState[servo]}
                    </label>
                    <input
                        type="range"
                        id={servo}
                        min="0"
                        max="180"
                        bind:value={jointState[servo]}
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
                        bind:checked={digitalPinState[doPin]}
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
                            digitalPinState[diPin] ? 'bg-green-500' : 'bg-red-500'
                        } text-white`}
                    >
                        {digitalPinState[diPin] ? 'ON' : 'OFF'}
                    </span>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    /* Add any additional styles if necessary */
</style>
