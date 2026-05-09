import { ComponentType } from "react";

type TabId = "setup" | "flash" | "motors";
export type Tab = {
    id: TabId;
    label: string;
    component: ComponentType;
};