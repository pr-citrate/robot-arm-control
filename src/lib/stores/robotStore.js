import { writable } from 'svelte/store';

export const robotState = writable({
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
