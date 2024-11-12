<!-- src/routes/+page.svelte -->

<script lang="ts">
    import {onMount} from 'svelte';
    import {
        currentState,
        digitalInputKeys,
        digitalOutputKeys,
        jointKeys,
        type RobotState,
        targetState
    } from '$lib/stores/robotStore';
    import {invoke} from '@tauri-apps/api/core';
    import {writable} from 'svelte/store';

    let selectedPort: string = '';
    let baudRate: number = 9600; // 기본 보드레이트

    // 사용 가능한 시리얼 포트 목록
    const availablePorts = writable<string[]>([]);

    // 상태 메시지
    const statusMessage = writable<string>('');

    // 시리얼 포트 초기화 함수
    const initializePort = async () => {
        if (!selectedPort) {
            statusMessage.set('시리얼 포트를 선택해주세요.');
            return;
        }

        try {
            await invoke('initialize_serial', { port: selectedPort, baudRate: baudRate });
            statusMessage.set(`시리얼 포트 ${selectedPort}가 초기화되었습니다.`);
            // 시리얼 포트 초기화 후 로봇 상태 가져오기
            await fetchRobotState();
        } catch (error) {
            console.error('시리얼 포트 초기화 실패:', error);
            statusMessage.set(`시리얼 포트 ${selectedPort} 초기화에 실패했습니다.`);
        }
    };

    // 로봇 명령 전송 함수
    const updateRobot = async () => {
    try {
        await invoke('send_robot_commands', { robotState: $targetState });
        statusMessage.set('명령이 성공적으로 전송되었습니다.');
    } catch (error) {
        console.error('로봇 명령 전송 오류:', error);
        statusMessage.set('명령 전송에 실패했습니다.'+error);
    }
};


    // 로봇 상태 가져오기 함수
    const fetchRobotState = async () => {
        try {
            const state: RobotState = await invoke('read_robot_state');
            currentState.set(state);
            // 현재 상태를 목표 상태에 반영 (필요 시)
            // targetState.set(state);
        } catch (error) {
            console.error('로봇 상태 읽기 실패:', error);
            statusMessage.set('로봇 상태를 읽어오는 데 실패했습니다.');
        }
    };

    // 시리얼 포트 목록 가져오기 함수
    const listPorts = async () => {
        try {
            const ports: string[] = await invoke('list_serial_ports');
            availablePorts.set(ports);
        } catch (error) {
            console.error('시리얼 포트 목록 가져오기 실패:', error);
            statusMessage.set('시리얼 포트 목록을 가져오는 데 실패했습니다.');
        }
    };

    // 컴포넌트 마운트 시 초기화 작업 수행
    onMount(async () => {
        await listPorts();

        // 사용 가능한 포트가 있을 경우 자동 선택
        availablePorts.subscribe(ports => {
            if (ports.length > 0 && !selectedPort) {
                selectedPort = ports[0];
            }
        });

        // 시리얼 포트 초기화
        await initializePort();

        // 로봇 상태를 주기적으로 가져오기
        setInterval(fetchRobotState, 1000);
    });
</script>

<div class="min-h-screen bg-sky-light text-slate-800 p-8">
    <h1 class="text-4xl font-bold mb-6 text-sky-dark">6-Axis Robot Arm Controller</h1>

    <!-- 시리얼 포트 설정 -->
    <div class="mb-8 bg-white shadow-md rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-sky-dark">시리얼 포트 설정</h2>
        <div class="flex items-center space-x-4">
            <div>
                <label for="serialPort" class="block text-sky-dark mb-2">시리얼 포트:</label>
                <select
                    id="serialPort"
                    bind:value={selectedPort}
                    class="w-full border border-sky-dark rounded p-2"
                >
                    <option value="" disabled>포트를 선택하세요</option>
                    {#each $availablePorts as port}
                        <option value={port}>{port}</option>
                    {/each}
                </select>
            </div>
            <div>
                <label for="baudRate" class="block text-sky-dark mb-2">보드레이트:</label>
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
                    초기화
                </button>
            </div>
        </div>
        {#if $statusMessage}
            <p class="mt-4 text-sm text-red-500">{$statusMessage}</p>
        {/if}
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <!-- 서보 컨트롤 -->
        <div class="bg-white shadow-md rounded-lg p-6">
            <h2 class="text-2xl font-semibold mb-4 text-sky-dark">서보 컨트롤</h2>
            {#each jointKeys as servo}
                <div class="mb-4">
                    <label class="block text-sky-dark mb-2" for={servo}>
                        {servo}: {$currentState[servo]} → {$targetState[servo]}
                    </label>
                    <input
                        type="range"
                        id={servo}
                        min="0"
                        max="180"
                        bind:value={$targetState[servo]}
                        class="w-full accent-sky-400"
                        on:input={updateRobot}
                    />
                </div>
            {/each}
            <div class="mb-4">
                <label class="block text-sky-dark mb-2" for="robot_speed">
                    로봇 속도: {$currentState.robot_speed} → {$targetState.robot_speed}
                </label>
                <input
                    type="range"
                    id="robot_speed"
                    min="0"
                    max="100"
                    bind:value={$targetState.robot_speed}
                    class="w-full accent-sky-400"
                    on:input={updateRobot}
                />
            </div>
        </div>

        <!-- 디지털 출력 -->
        <div class="bg-white shadow-md rounded-lg p-6">
            <h2 class="text-2xl font-semibold mb-4 text-sky-dark">디지털 출력</h2>
            {#each digitalOutputKeys as doPin}
                <div class="flex items-center mb-4">
                    <input
                        type="checkbox"
                        id={doPin}
                        bind:checked={$targetState[doPin]}
                        class="mr-2 accent-sky-400"
                        on:change={updateRobot}
                    />
                    <label class="text-sky-dark" for={doPin}>{doPin}</label>
                </div>
            {/each}
        </div>
    </div>

    <!-- 디지털 입력 -->
    <div class="mt-8 bg-white shadow-md rounded-lg p-6">
        <h2 class="text-2xl font-semibold mb-4 text-sky-dark">디지털 입력</h2>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
            {#each digitalInputKeys as diPin}
                <div class="flex items-center">
                    <span class="text-sky-dark mr-2">{diPin}:</span>
                    <span
                        class={`px-2 py-1 rounded ${
                            $currentState[diPin] ? 'bg-sky-400' : 'bg-red-400'
                        } text-white`}
                    >
                        {$currentState[diPin] ? 'ON' : 'OFF'}
                    </span>
                </div>
            {/each}
        </div>
    </div>
</div>
