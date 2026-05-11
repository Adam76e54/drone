import { useState } from "react";
import  Box  from "@mui/material/Box"
import { Typography } from "@mui/material";
import "./App.css";
import TabBar, { type ConfigTab } from "../tab-bar/TabBar.tsx"
import SetupPage from "../setup/Setup.tsx";
import FlasherPage from "../flasher/Flasher.tsx";


function App() {
  const [tab, setTab] = useState<ConfigTab>('setup');

  return (
    <Box sx={{ minHeight: '100vh', bgcolor: 'background.default' }}>
      <TabBar tab={tab} onTabChange={setTab}/>

      <Box component="main" sx={{ p: 2 }}>
        {tab === 'setup' && <SetupPage />}
        {tab === 'receiver' && <Typography color="text.primary">Receiver page</Typography>}
        {tab === 'motors' && <Typography color="text.primary">Motors page</Typography>}
        {tab === 'pid' && <Typography color="text.primary">PID page</Typography>}
        {tab === 'rates' && <Typography color="text.primary">Rates page</Typography>}
        {tab === 'blackbox' && <Typography color="text.primary">Blackbox page</Typography>}
        {tab === 'flashing' && <FlasherPage />}
      </Box>
    </Box>
  );
}

export default App;
