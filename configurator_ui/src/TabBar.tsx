import type { TabId } from "./ConfiguratorPage";

type TabBarProps = {
    activeTab: TabId;
    onTabChange: (tabId: TabId) => void;
};

export function TabBar({ activeTab, onTabChange }: TabBarProps) {
    return (
        <nav className="sidebar" aria-label="Configurator Tabs">
            <button
                role="tab"
                className={activeTab === "Setup" ? "tab active" : "tab"}
                onClick={() => onTabChange("Setup")}
            >
                Setup
            </button>

            <button
                role="tab"
                className={activeTab === "PID" ? "tab active" : "tab"}
                onClick={() => onTabChange("PID")}
            >
                PID
            </button>

            <button
                role="tab"
                className={activeTab === "Receiver" ? "tab active" : "tab"}
                onClick={() => onTabChange("Receiver")}
            >
                Receiver
            </button>

            <button
                role="tab"
                className={activeTab === "Ports" ? "tab active" : "tab"}
                onClick={() => onTabChange("Ports")}
            >
                Ports
            </button>

            <button
                role="tab"
                className={activeTab === "Motors" ? "tab active" : "tab"}
                onClick={() => onTabChange("Motors")}
            >
                Motors
            </button>


        </nav>
    );

}