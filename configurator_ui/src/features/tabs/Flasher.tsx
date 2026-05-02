import "./Flasher.css";
import Button from "@mui/material/Button";
export function FlasherTab() {
    return (
        <Button variant="contained" onClick={() => alert("Flashing firmware...")}>
            Flash Firmware
        </Button>
    )
}