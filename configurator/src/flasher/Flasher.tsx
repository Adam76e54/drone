import { invoke } from "@tauri-apps/api/core";
import { Box, Button, Card } from "@mui/material";

export default function FlasherPage() {
    const handleFlash = async () => {
        try {
            const result = await invoke<string>('flash_firmware', {
                port: 'COMx',
                path: 'path',
            });

            console.log('Flash result:', result);
        } catch (error){
            console.error('Flash failed', error);
        }
    };

    return (
        <Box>
            <Card>
                <Button onClick={handleFlash}>
                    Flash
                </Button>
            </Card>
        </Box>
    );
}