import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri'
import { CpuData, MemData, NvStats } from '../../scripts/interfaces'
import Cpu from "../../components/Cpu/Cpu";
import Nvidia from "../../components/Nvidia/Nvidia";


export default function Main(Data: {
  cpu: CpuData | null,
  memory: MemData | null,
  nvidia: NvStats | null
}) {

  let cpu = Data.cpu
  let memory = Data.memory
  let nvidia = Data.nvidia
  return (<>
    {cpu && memory ? <Cpu cpu={cpu} memory={memory} /> : null}
    {nvidia ? <Nvidia nvidia={nvidia} /> : null}
  </>);
}