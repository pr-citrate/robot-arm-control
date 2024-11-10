import {type Writable, writable} from 'svelte/store';

interface robotStateInterface {
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

export const robotState: Writable<robotStateInterface> = writable({
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
});
