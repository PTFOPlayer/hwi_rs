import { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri'
import {Msr, NvStats } from '../../scripts/interfaces'
import Cpu from "../../components/Cpu/Cpu";
import Nvidia from "../../components/Nvidia/Nvidia";


export default function Main(Data: {
  msr: Msr | null;
  nvidia: NvStats | null
}) {

  return (<>
    {Data.msr ? <Cpu msr={Data.msr} /> : null}
    {Data.nvidia ? <Nvidia nvidia={Data.nvidia} /> : null}
  </>);
}