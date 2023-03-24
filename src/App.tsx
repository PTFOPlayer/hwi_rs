import { useState, useEffect } from "react";
import "./App.css";
import {invoke} from '@tauri-apps/api/tauri'

function App() {
  const [cpu, setCpu] = useState("")

  async function setters() {
    setCpu(await invoke("tauri_get_cpu")); 
  }

  useEffect(()=> {
    const timer = setInterval(setters, 1000)
    return () => {
      clearInterval(timer)
    }
  }, [])

  return (
    <div className="container">
      <h1> {cpu} </h1>
    </div>
  );
}

export default App;
