import { SetupTab } from "./SetupTab";
import { PidTab } from "./PidTab";
import { ReceiverTab } from "./ReceiverTab";
import { PortsTab } from "./PortsTab";
import { MotorsTab } from "./MotorsTab";
import { FlasherTab } from "./Flasher";

export const tabs = [
    {id: "Setup", label: "Setup", component: SetupTab},
    {id: "PID", label: "PID", component: PidTab},
    {id: "Receiver", label: "Receiver", component: ReceiverTab},
    {id: "Ports", label: "Ports", component: PortsTab},
    {id: "Motors", label: "Motors", component: MotorsTab},
    {id: "Flasher", label: "Flasher", component: FlasherTab},
] as const;

/**
 * This looks weird but: "typeof tabs" just fetches the type of that tabs array
 * the [number] is syntax for "union of all elements" so tabs[number] is tab[0] | tab[1] | tab[2] | ... 
 * then the ["id"] fetches the property inside of that tab[i] element so we end up with "Setup" | "PID" | "Receiver" ...
 * Note: (typeof tabs)[number] is valid, but tabs[number] is not. [number] works on types, not values.
 */
export type TabId = (typeof tabs)[number]["id"];