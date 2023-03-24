import { useState, useEffect } from "react";
import "./App.css";
import {invoke} from '@tauri-apps/api/tauri'

interface CacheData {
  size: number,
  level: number,
  cache_type: String
}

interface CpuData {
  name: String,
  logical_cores: number,
  physical_cores: number,
  power: number,
  voltage: number,
  frequency: Array<String>,
  load: number,
  temperature: number,
  cache: Array<CacheData>,
  hyper_threading: number,
}

function App() {
  const [cpu, setCpu] = useState<CpuData | null>(null)

  async function setters() {
    await invoke("tauri_get_cpu").then(res => setCpu(res as CpuData)).catch(() => setCpu(null));
  }

  useEffect(()=> {
    const timer = setInterval(setters, 500)
    return () => {
      clearInterval(timer)
    }
  }, [])

  return (
    <div className="container">
      <h1> {cpu?.power} </h1>
    </div>
  );
}

export default App;
