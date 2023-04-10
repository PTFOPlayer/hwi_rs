import { useState, useEffect } from "react";
import "./App.css";
import { invoke } from '@tauri-apps/api/tauri'
import { CpuData, NvSpec, NvStats } from './scripts/interfaces'
import Cpu from "./components/Cpu/Cpu";
import Nvidia from "./components/Nvidia/Nvidia";

function App() {
  const [nvidia, setNvidia] = useState<NvStats | null>(null)
  const [cpu, setCpu] = useState<CpuData | null>(null)
  
  async function setters() {
    await invoke("tauri_get_nv")
      .then(res => setNvidia(res as NvStats))
      .catch(() => setNvidia(null));
    await invoke("tauri_get_cpu")
      .then(res => setCpu(res as CpuData))
      .catch(() => setCpu(null));

  }

  useEffect(() => {
    const timer = setInterval(setters, 500)
    return () => {
      clearInterval(timer)
    }
  }, [])

  let handle_click = async () => {
    await invoke("second_window")
  }

  return (
    <div>
      <button onClick={handle_click}>new_window</button>
      {cpu ? <Cpu cpu={cpu}/> : null}
      {nvidia ? <Nvidia nvidia={nvidia}/> : null}
     </div>
  );
}

export default App;
