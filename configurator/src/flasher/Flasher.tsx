import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";

import { Box, Button, Card, Select, SelectChangeEvent, MenuItem, Stack, TextField, Typography } from "@mui/material";

type ConnectionMode = "usb1" | "usb2";

export default function FlasherPage() {
    const [port, setPort] = useState<ConnectionMode>("usb1");
    const [firmwarePath, setFirmwarePath] = useState("");
    const [stm32CubePath, setStm32CubePath] = useState(
        "C:\\Program Files\\STMicroelectronics\\STM32Cube\\STM32CubeProgrammer\\bin\\STM32_Programmer_CLI.exe");
    
    const handlePickFirmware = async () => {
        try {
            const selected = await pickFile("elf");

            if(selected !== null) {
                setFirmwarePath(selected);
            }
        } catch (error) {
            console.error("Failed to pick firmware file", error);
        }
    };

    const handlePickStm32CubePath = async () => {
        try {
            const selected = await pickFile("exe");


            if(selected !== null) {
                setStm32CubePath(selected);
            }
        } catch (error) {
            console.error("Filaed to pick STM32CubeProgrammer.exe path", error);
        }
    }

    const handleFlash = async () => {
        try {
            const result = await flashFirmware(firmwarePath, port, stm32CubePath);

            console.log('Flash result:', result);
        } catch (error){
            console.error('Flash failed', error);
        }
    };

    const handlePickUsb = (event: SelectChangeEvent<ConnectionMode>) => {
        setPort(event.target.value as ConnectionMode);
    }

    return (
        <Box sx={{p:2}}>
            <Stack spacing={2}>
                <Card sx={{p:2}}>
                    <Stack spacing={2}>
                        <Typography variant="h6">Firmware Flashing</Typography>
                    
                        <TextField
                            disabled
                            label="Firmware file"
                            value={firmwarePath}
                            fullWidth
                            slotProps={{
                                input: {
                                    readOnly: true,
                                },
                            }}
                        />

                        <Stack direction="row" spacing={2}>
                            <Button variant="outlined" onClick={handlePickFirmware}>
                                Select Firmware
                            </Button>

                            <Button variant="outlined" onClick={handlePickStm32CubePath}>
                                Select STM32CubeProgrammer
                            </Button>

                            <Button
                                variant="contained"
                                onClick={handleFlash}
                                disabled={!firmwarePath}
                            >
                                Flash
                            </Button>

                                <Select
                                labelId="port-select-label"
                                value={port}
                                label="Connection"
                                onChange={handlePickUsb}
                                >
                                <MenuItem value="usb1">USB1</MenuItem>
                                <MenuItem value="usb2">USB2</MenuItem>
                                </Select>
                        </Stack>
                    </Stack>
                </Card>
            </Stack>
        </Box>
    );
}

async function pickFile(extension: string): Promise<string | null> {
    const selected = await open({
        multiple: false, 
        directory: false,
        title: "Select firmware",
    });

    if (typeof selected === "string") {
        return selected;
    }

    return null;
}

async function flashFirmware(firmwarePath: string, port: string, stm32CubePath: string) {
    return await invoke<string>("flash_firmware", {
        rawFirmware: firmwarePath,
        port: port, 
        stm32CubePath: stm32CubePath,
    });
}