import { createTheme } from "@mui/material/styles";

export const theme = createTheme({

    palette: {
        mode: "dark",
        primary: { main : "#ff90cb" },        
        secondary: { main: "#dc004e"},
        background: {
            default: "#121212",
            paper: "#1d1d1d"
        },
    },
});