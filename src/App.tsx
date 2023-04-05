import { useState, useEffect } from "react";
import "./App.css";
import { invoke } from '@tauri-apps/api/tauri'
import { CpuData, NvSpec } from './scripts/interfaces'
import Cpu from "./components/Cpu/Cpu";

function App() {
  const [nvidia, setNvidia] = useState<NvSpec | null>(null)
  const [cpu, setCpu] = useState<CpuData | null>(null)
  
  async function setters() {
    await invoke("tauri_get_nv")
      .then(res => setNvidia(res as NvSpec))
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

  return (
    <div>
      {cpu ? <Cpu cpu={cpu}/> : null }
     </div>
  );
}

export default App;
