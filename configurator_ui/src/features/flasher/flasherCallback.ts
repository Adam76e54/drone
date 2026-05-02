type FlashProgress = 
    | { stage: "idle" }
    | { stage: "request-device" }
    | { stage: "enter-bootloader" }
    | { stage: "erasing"; percent?: number }
    | { stage: "writing"; percent: number }
    | { stage: "verifying"; percent?: number }
    | { stage: "rebooting" }
    | { stage: "done" }
    | { stage: "error"; message: string };


function todo(message = "Not implemented") : never {
    throw new Error(message);
}

async function onFlashClick() : Promise<void>{
    todo("Need to implment if selected firmware check");

    /**
     * if(no firmware) { setFlasherState({stage: "error", message: "choose firmware"})} or something like that
     */

    try {
        
    }
}

