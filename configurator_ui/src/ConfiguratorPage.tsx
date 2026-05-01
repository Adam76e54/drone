import { useState } from "react";
import { TabBar } from "./TabBar";

import { tabs, type TabId } from "./features/tabs/Tabs";
import "./ConfiguratorPage.css";



export function ConfiguratorPage() {
    // Use the useState hook to manage the active tab state
    // setActiveTab(id) will switch the active tab to the id'd tab
    const [activeTab, setActiveTab] = useState<TabId>("Setup");

    // Fetch active tab
    const activeTabConfiguration = tabs.find((tab) => tab.id === activeTab);
    // Fetch the active html component. 
    // NOTE: ?. is a safe member access operator, returns null if no such member exists. 
    const ActiveTabComponent = activeTabConfiguration?.component;


    return (
        <div>
            <main className="configurator-layout">
                <h1 className="header">Configurator</h1>

                <TabBar activeTab={activeTab} onTabChange={setActiveTab} />

                <section className="content-area" style={{ marginLeft: "1rem" }}>
                    {/* Render the active tab */}
                    {ActiveTabComponent ? <ActiveTabComponent /> : null}
                </section>
            </main>
        </div>

    );
}