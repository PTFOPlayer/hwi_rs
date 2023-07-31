import Main from "./pages/Main/Main";
import "./App.scss"
import { HashRouter as Router, Routes, Route } from "react-router-dom";
import NvidiaOc from "./pages/NvidiaOC/NvidiaOc";
import { invoke } from "@tauri-apps/api";
import { useState, useEffect, memo } from "react";
import { NvStats, CpuData, MemData } from "./scripts/interfaces";
import Bar from "./components/Bar/Bar";
import Sidebar from "./components/Sidebar/Sidebar";
function App() {

  const [nvidia, setNvidia] = useState<NvStats | null>(null)
  const [cpu, setCpu] = useState<CpuData | null>(null)
  const [memory, setMemory] = useState<MemData | null>(null)

  async function setters() {
    await invoke("tauri_get_nv")
      .then(res => setNvidia(res as NvStats))
      .catch(() => setNvidia(null));
    await invoke("tauri_get_cpu")
      .then(res => setCpu(res as CpuData))
      .catch(() => setCpu(null));
    await invoke("tauri_get_memory")
      .then(res => setMemory(res as MemData))
      .catch(() => setMemory(null));
  }

  useEffect(() => {
    const timer = setInterval(setters, 1000)
    return () => {
      clearInterval(timer)
    }
  }, [])

  return (
    <div className="app">
      <Bar />
      <div className="core">
        <Sidebar />
        <div className="element">
          <Router>
            <Routes>
              <Route path="/" element={<Main cpu={cpu} memory={memory} nvidia={nvidia} />} />
              <Route path="/nvoc" element={<NvidiaOc nvidia={nvidia} />} />
            </Routes>
          </Router>
        </div>
      </div>
    </div>
  );
}

export default App;
