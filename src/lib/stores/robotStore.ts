// src/lib/stores/robotStore.ts

import {writable, type Writable} from 'svelte/store';

// Joint 및 Digital Pin 키 정의
export type JointKeys = 'joint_1' | 'joint_2' | 'joint_3' | 'joint_4' | 'joint_5' | 'joint_6';
export type DigitalOutputKeys = 'digital_output_1' | 'digital_output_2' | 'digital_output_3';
export type DigitalInputKeys = 'digital_input_1' | 'digital_input_2' | 'digital_input_3';

// 키 배열 정의
export const jointKeys: JointKeys[] = ['joint_1', 'joint_2', 'joint_3', 'joint_4', 'joint_5', 'joint_6'];
export const digitalOutputKeys: DigitalOutputKeys[] = ['digital_output_1', 'digital_output_2', 'digital_output_3'];
export const digitalInputKeys: DigitalInputKeys[] = ['digital_input_1', 'digital_input_2', 'digital_input_3'];

// Robot 상태 인터페이스 정의
export type RobotState = Record<JointKeys, number> &
    Record<DigitalOutputKeys, boolean> &
    Record<DigitalInputKeys, boolean> & {
    robot_speed: number;
};

// **현재 상태(Current State)**
export const currentState: Writable<RobotState> = writable({
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
    robot_speed: 50,
});

// **목표 상태(Target State)**
export const targetState: Writable<RobotState> = writable({
    joint_1: 90,
    joint_2: 90,
    joint_3: 90,
    joint_4: 90,
    joint_5: 90,
    joint_6: 90,
    digital_input_1: false, // 디지털 입력은 읽기 전용이라면 제외할 수 있습니다.
    digital_input_2: false,
    digital_input_3: false,
    digital_output_1: false,
    digital_output_2: false,
    digital_output_3: false,
    robot_speed: 50,
});
