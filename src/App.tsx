import Main from "./pages/Main/Main";
import "./App.scss"
import { HashRouter as Router, Routes, Route } from "react-router-dom";
import { invoke } from "@tauri-apps/api";
import { useState, useEffect} from "react";
import { NvStats,  Msr } from "./scripts/interfaces";
import Bar from "./components/Bar/Bar";
import Sidebar from "./components/Sidebar/Sidebar";
function App() {

  const [nvidia, setNvidia] = useState<NvStats | null>(null)
  const [msr, setMsr] = useState<Msr | null>(null)

  async function setters() {
    await invoke("tauri_get_nv")
      .then(res => setNvidia(res as NvStats))
      .catch(() => setNvidia(null));
    await invoke("tauri_get_msr_data")
      .then(res => setMsr(res as Msr))
      .catch(() => setMsr(null));
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
              <Route path="/" element={<Main  nvidia={nvidia} msr={msr} />} />
            </Routes>
          </Router>
        </div>
      </div>
    </div>
  );
}

export default App;
