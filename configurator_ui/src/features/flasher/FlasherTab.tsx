import { useMemo, useState } from "react";
import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  CardHeader,
  Checkbox,
  Chip,
  Container,
  FormControlLabel,
  LinearProgress,
  Paper,
  Stack,
  Typography,
} from "@mui/material";

type FlashStage =
  | "idle"
  | "preparing"
  | "erasing"
  | "programming"
  | "verifying"
  | "rebooting"
  | "done"
  | "failed";

interface FlasherState {
  device: {
    connected: boolean;
    portLabel: string | null;
    mode: "disconnected" | "runtime" | "bootloader" | "dfu";
    boardName: "Matrix 1S Brushless Flight Controller";
  };
  firmware: {
    localFile: File | null;
    localFileName: string | null;
  };
  options: {
    fullChipErase: boolean;
    verifyAfterWrite: boolean;
  };
  job: {
    stage: FlashStage;
    progress: number;
    logLines: string[];
    errorMessage: string | null;
  };
}

const initialState: FlasherState = {
  device: {
    connected: false,
    portLabel: "USB DFU",
    mode: "dfu",
    boardName: "Matrix 1S Brushless Flight Controller",
  },
  firmware: {
    localFile: null,
    localFileName: null,
  },
  options: {
    fullChipErase: true,
    verifyAfterWrite: true,
  },
  job: {
    stage: "idle",
    progress: 0,
    logLines: ["Waiting for firmware file..."],
    errorMessage: null,
  },
};

export function FlasherTab() {
  const [state, setState] = useState<FlasherState>(initialState);

  const canFlash = useMemo(() => {
    const busyStages: FlashStage[] = [
      "preparing",
      "erasing",
      "programming",
      "verifying",
      "rebooting",
    ];

    return (
      state.device.connected &&
      state.device.mode === "dfu" &&
      state.firmware.localFile !== null &&
      !busyStages.includes(state.job.stage)
    );
  }, [state]);

  function appendLog(line: string) {
    setState((current) => ({
      ...current,
      job: {
        ...current.job,
        logLines: [...current.job.logLines, line],
      },
    }));
  }

  function setStage(stage: FlashStage, progress: number) {
    setState((current) => ({
      ...current,
      job: {
        ...current.job,
        stage,
        progress,
      },
    }));
  }

  function setError(message: string) {
    setState((current) => ({
      ...current,
      job: {
        ...current.job,
        stage: "failed",
        errorMessage: message,
      },
    }));
  }

  function clearError() {
    setState((current) => ({
      ...current,
      job: {
        ...current.job,
        errorMessage: null,
      },
    }));
  }

  function handleChooseFile(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0] ?? null;

    setState((current) => ({
      ...current,
      firmware: {
        localFile: file,
        localFileName: file?.name ?? null,
      },
      job: {
        ...current.job,
        stage: "idle",
        progress: 0,
        errorMessage: null,
        logLines: file
          ? [`Selected firmware file: ${file.name}`]
          : ["Waiting for firmware file..."],
      },
    }));
  }

  async function handleFlash() {
    if (!state.firmware.localFile) {
      setError("No firmware file selected.");
      return;
    }

    clearError();
    setStage("preparing", 5);
    appendLog("Opening flash session...");
    await wait(400);

    if (state.options.fullChipErase) {
      setStage("erasing", 20);
      appendLog("Erasing flash...");
      await wait(900);
    } else {
      appendLog("Skipping full chip erase.");
    }

    setStage("programming", 60);
    appendLog(`Programming ${state.firmware.localFileName ?? "firmware"}...`);
    await wait(1200);

    if (state.options.verifyAfterWrite) {
      setStage("verifying", 85);
      appendLog("Verifying written image...");
      await wait(700);
    } else {
      appendLog("Verification disabled.");
    }

    setStage("rebooting", 95);
    appendLog("Rebooting board...");
    await wait(500);

    setStage("done", 100);
    appendLog("Flash complete.");
  }

  return (
    <Container maxWidth="md" sx={{ py: 3 }}>
      <Stack spacing={3}>
        <Typography variant="h4">Firmware Flasher</Typography>

        <Card variant="outlined">
          <CardHeader title="Board status" />
          <CardContent>
            <Stack direction={{ xs: "column", sm: "row" }} spacing={1.5}>
              <Chip
                color={state.device.connected ? "success" : "default"}
                label={
                  state.device.connected
                    ? `Connected: ${state.device.portLabel ?? "Unknown port"}`
                    : "Disconnected"
                }
              />
              <Chip variant="outlined" label={`Mode: ${state.device.mode}`} />
              <Chip variant="outlined" label={state.device.boardName} />
            </Stack>
          </CardContent>
        </Card>

        <Card variant="outlined">
          <CardHeader title="Firmware file" />
          <CardContent>
            <Stack spacing={2}>
              <Button variant="outlined" component="label">
                Choose local firmware
                <input hidden type="file" accept=".bin,.hex" onChange={handleChooseFile} />
              </Button>

              <Typography variant="body2" color="text.secondary">
                {state.firmware.localFileName ?? "No file selected"}
              </Typography>
            </Stack>
          </CardContent>
        </Card>

        <Card variant="outlined">
          <CardHeader title="Flash options" />
          <CardContent>
            <Stack>
              <FormControlLabel
                control={
                  <Checkbox
                    checked={state.options.fullChipErase}
                    onChange={(event) =>
                      setState((current) => ({
                        ...current,
                        options: {
                          ...current.options,
                          fullChipErase: event.target.checked,
                        },
                      }))
                    }
                  />
                }
                label="Full chip erase"
              />

              <FormControlLabel
                control={
                  <Checkbox
                    checked={state.options.verifyAfterWrite}
                    onChange={(event) =>
                      setState((current) => ({
                        ...current,
                        options: {
                          ...current.options,
                          verifyAfterWrite: event.target.checked,
                        },
                      }))
                    }
                  />
                }
                label="Verify after write"
              />
            </Stack>
          </CardContent>
        </Card>

        <Card variant="outlined">
          <CardHeader title="Flash job" />
          <CardContent>
            <Stack spacing={2}>
              <Box>
                <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
                  Stage: {state.job.stage}
                </Typography>
                <LinearProgress variant="determinate" value={state.job.progress} />
              </Box>

              {state.job.errorMessage && (
                <Alert severity="error">{state.job.errorMessage}</Alert>
              )}

              <Stack direction={{ xs: "column", sm: "row" }} spacing={2}>
                <Button variant="contained" disabled={!canFlash} onClick={handleFlash}>
                  Flash firmware
                </Button>

                <Button
                  variant="outlined"
                  onClick={() => setState(initialState)}
                >
                  Reset page
                </Button>
              </Stack>

              <Paper
                variant="outlined"
                sx={{
                  p: 2,
                  minHeight: 160,
                  bgcolor: "background.default",
                  overflow: "auto",
                }}
              >
                <Typography variant="subtitle2" sx={{ mb: 1 }}>
                  Session log
                </Typography>

                <Stack spacing={0.5}>
                  {state.job.logLines.map((line, index) => (
                    <Typography
                      key={`${line}-${index}`}
                      variant="body2"
                      sx={{ fontFamily: "monospace" }}
                    >
                      {line}
                    </Typography>
                  ))}
                </Stack>
              </Paper>
            </Stack>
          </CardContent>
        </Card>
      </Stack>
    </Container>
  );
}

function wait(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}