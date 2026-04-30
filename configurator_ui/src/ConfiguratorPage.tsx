import { useState } from "react";
import { TabBar } from "./TabBar";
import { SetupTab } from "./features/tabs/SetupTab";
import { PidTab } from "./features/tabs/PidTab";
import { ReceiverTab } from "./features/tabs/ReceiverTab";
import { PortsTab } from "./features/tabs/PortsTab";
import { MotorsTab } from "./features/tabs/MotorsTab";
import "./ConfiguratorPage.css";

export type TabId = "Setup" | "PID" | "Receiver" | "Ports" | "Motors";

export function ConfiguratorPage() {
    const [activeTab, setActiveTab] = useState<TabId>("Setup");

    return (
        <div>

            <main className="configurator-layout">
                <h1 className="header">Configurator</h1>

                <TabBar activeTab={activeTab} onTabChange={setActiveTab} />

                <section className="content-area" style={{ marginLeft: "1rem" }}>
                    {activeTab === "Setup" && <SetupTab />}
                    {activeTab === "PID" && <PidTab />}
                    {activeTab === "Receiver" && <ReceiverTab />}
                    {activeTab === "Ports" && <PortsTab />}
                    {activeTab === "Motors" && <MotorsTab />}
                </section>
            </main>
        </div>

    );
}