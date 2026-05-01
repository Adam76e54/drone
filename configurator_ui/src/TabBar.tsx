import type { TabId } from "./features/tabs/Tabs";
import { tabs } from "./features/tabs/Tabs";

// Define a tab bar properties type that includes the active tab and a callback function for changing tabs.
type TabBarProps = {
    activeTab: TabId;
    onTabChange: (tabId: TabId) => void;
};

/**
 * Renders the tab bar 
 * The TabBar component receives the currently active tab and a callback function to handle tab changes. 
    
 * @returns The rendered TabBar component.
 */
export function TabBar({ activeTab, onTabChange }: TabBarProps) { 
    return (
        <nav className="sidebar" aria-label="Configurator Tabs">
            {// Render a button for each tab in tabs[] using map(), which just iterates over all elements.
            // it's like for(auto tab : tabs) in C++
            tabs.map((tab) => (
                <button
                    role="tab"
                    type="button"
                    aria-selected={activeTab === tab.id}
                    className={activeTab === tab.id ? "tab active" : "tab"}
                    onClick={() => onTabChange(tab.id)}
                >
                    {tab.label}
                </button>
            ))}
        </nav>
    );
}
