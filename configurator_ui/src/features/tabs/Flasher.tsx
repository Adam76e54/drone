import "./Flasher.css";
export function FlasherTab() {
    return (
        <button className="flasher-button" onClick={() => alert("Flashing firmware...")}>
            Flash Firmware
        </button>
    )
}