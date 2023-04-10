import { useState, useEffect } from "react";
import "./App.css";
import { invoke } from '@tauri-apps/api/tauri'
import { CpuData, NvSpec, NvStats } from '../scripts/interfaces'

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
      Lorem, ipsum dolor sit amet consectetur adipisicing elit. Quidem dolorem temporibus sint dolorum similique velit accusamus possimus saepe labore quibusdam neque expedita ducimus, explicabo, magni voluptatibus necessitatibus, ullam iure accusantium.
      
     </div>
  );
}

export default App;
