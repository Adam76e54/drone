import { ConfiguratorPage } from '../ConfiguratorPage';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { theme } from '../theme';

function App() {

  return (
    <ThemeProvider theme={theme}>
      <ConfiguratorPage />
    </ThemeProvider>
  )
}

export default App
