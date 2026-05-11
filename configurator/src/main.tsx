import React from "react";
import ReactDOM from "react-dom/client";
import App from "./app/App";
import theme from './themes/themes.ts'
import { ThemeProvider } from "@emotion/react";
import { CssBaseline } from "@mui/material";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <CssBaseline>
        <App /> 
      </CssBaseline>
    </ThemeProvider>
  </React.StrictMode>,
);