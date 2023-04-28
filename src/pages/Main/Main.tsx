import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri'
import { CpuData, MemData, NvStats } from '../../scripts/interfaces'
import Cpu from "../../components/Cpu/Cpu";
import Nvidia from "../../components/Nvidia/Nvidia";


export default function Main() {
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
    const timer = setInterval(setters, 500)
    return () => {
      clearInterval(timer)
    }
  }, [])
  return (<>
    {cpu && memory ? <Cpu cpu={cpu} memory={memory} /> : null}
    {nvidia ? <Nvidia nvidia={nvidia} /> : null}
  </>);
}