<!-- src/routes/index.svelte -->

<script lang="ts">
    import { onMount } from "svelte";
    import { robotState } from '$lib/stores/robotStore';
    import { invoke } from "@tauri-apps/api/core";
    import { writable } from 'svelte/store';

    interface StateInterface {
        joint_1: number;
        joint_2: number;
        joint_3: number;
        joint_4: number;
        joint_5: number;
        joint_6: number;
        digital_input_1: boolean;
        digital_input_2: boolean;
        digital_input_3: boolean;
        digital_output_1: boolean;
        digital_output_2: boolean;
        digital_output_3: boolean;
        robotSpeed: number;
    }

    let localState: StateInterface = {
        joint_1: 90,
        joint_2: 90,
        joint_3: 90,
        joint_4: 90,
        joint_5: 90,
        joint_6: 90,
        digital_input_1: false,
        digital_input_2: false,
        digital_input_3: false,
        digital_output_1: false,
        digital_output_2: false,
        digital_output_3: false,
        robotSpeed: 50,
    };

    const getjoint_ointState = (state: StateInterface): Record<string, number> => {
        return {
            joint_1: state.joint_1,
            joint_2: state.joint_2,
            joint_3: state.joint_3,
            joint_4: state.joint_4,
            joint_5: state.joint_5,
            joint_6: state.joint_6,
        }
    }

    const getdigital_input_gitalPinState = (state: StateInterface): Record<string, boolean> => {
        return {
            digital_input_1: state.digital_input_1,
            digital_input_2: state.digital_input_2,
            digital_input_3: state.digital_input_3,
            digital_output_1: state.digital_output_1,
            digital_output_2: state.digital_output_2,
            digital_output_3: state.digital_output_3,
        }
    }

    let jointState: Record<string, number> = getjoint_ointState(localState);
    let digitalPinState: Record<string, boolean> = getdigital_input_gitalPinState(localState);

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
            jointState = getjoint_ointState(localState);
            digitalPinState = getdigital_input_gitalPinState(localState);
        } catch (error) {
            console.error('Error reading robot state:', error);
            statusMessage.set('Failed to read robot state.' + error);
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
            await invoke('initialize_serial', { port: selectedPort, baudRate: baudRate });
            statusMessage.set(`Serial port ${selectedPort} initialized.`);
            // Optionally, fetch robot state immediately after initialization
            await fetchRobotState();
        } catch (error) {
            console.error('Failed to initialize serial port:', error);
            statusMessage.set(`Failed to initialize serial port ${selectedPort}.` + error);
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
            <div>
                <button
                    on:click={initializePort}
                    class="bg-sky-400 text-white px-4 py-2 rounded hover:bg-sky-500"
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
            {#each ['joint_1', 'joint_2', 'joint_3', 'joint_4', 'joint_5', 'joint_6'] as servo}
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
                        class="w-full accent-sky-400"
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
                    class="w-full accent-sky-400"
                    on:input={updateRobot}
                />
            </div>
        </div>

        <!-- digital_input_gital Outputs -->
        <div class="bg-white shadow-md rounded-lg p-6">
            <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Digital Outputs</h2>
            {#each ['digital_output_1', 'digital_output_2', 'digital_output_3'] as doPin}
                <div class="flex items-center mb-4">
                    <input
                        type="checkbox"
                        id={doPin}
                        bind:checked={digitalPinState[doPin]}
                        class="mr-2 accent-sky-400"
                        on:change={updateRobot}
                    />
                    <label class="text-sky-dark" for={doPin}>{doPin}</label>
                </div>
            {/each}
        </div>
    </div>

    <!-- digital_input_gital Inputs -->
    <div class="mt-8 bg-white shadow-md rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-sky-dark">Digital Inputs</h2>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
            {#each ['digital_input_1', 'digital_input_2', 'digital_input_3'] as diPin}
                <div class="flex items-center">
                    <span class="text-sky-dark mr-2">{diPin}:</span>
                    <span
                        class={`px-2 py-1 rounded ${
                            digitalPinState[diPin] ? 'bg-sky-400' : 'bg-red-400'
                        } text-white`}
                    >
                        {digitalPinState[diPin] ? 'ON' : 'OFF'}
                    </span>
                </div>
            {/each}
        </div>
    </div>
</div>
