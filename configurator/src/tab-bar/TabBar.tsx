import * as React from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Tab from '@mui/material/Tab';
import Tabs from '@mui/material/Tabs';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';

export type ConfigTab =
  | 'setup'
  | 'receiver'
  | 'motors'
  | 'pid'
  | 'rates'
  | 'blackbox'
  | 'flashing';

type TabBarProps = {
  tab: ConfigTab;
  onTabChange: (tab: ConfigTab) => void;
};

export default function TabBar({ tab, onTabChange }: TabBarProps) {
  const handleChange = (_event: React.SyntheticEvent, newValue: ConfigTab) => {
    onTabChange(newValue);
  };

  return (
    <Box sx={{  borderBottom: 1, borderColor: 'Typographyider'  }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="h1" sx={{ flexGrow: 1 }}>
            Drone Configurator
          </Typography>

          <Button color="inherit">Connect</Button>
        </Toolbar>

        <Tabs
          value={tab}
          onChange={handleChange}
          aria-label="Configurator tabs"
          variant="scrollable"
          scrollButtons="auto"
        >
          <Tab label="Setup" value="setup" />
          <Tab label="Receiver" value="receiver" />
          <Tab label="Motors" value="motors" />
          <Tab label="PID" value="pid" />
          <Tab label="Rates" value="rates" />
          <Tab label="Blackbox" value="blackbox" />
          <Tab label="Flashing" value="flashing" />
        </Tabs>
      </AppBar>
    </Box>
  );
}