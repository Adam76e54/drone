import type { Tab } from '../tab-bar/types.ts'

function FlasherPage() {
    return (
        <div>Flasher</div>
    );
}

export const Flasher: Tab = {
    id: "flash",
    label: "Flash", 
    component: FlasherPage,
}