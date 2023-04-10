import { NvStats } from "../../scripts/interfaces";

export default function Nvidia(nvData:{nvidia: NvStats}) {
    let NvSpec = nvData.nvidia.spec;
    let NvUtil = nvData.nvidia.util;
    return (
        <div className="cpu">
          <h1> GPU: {NvSpec.name} </h1>
          <div className="data">
            <div>
              <h3> General </h3>
              <p> Usage: {NvUtil.core_usage} % </p>
              <p> Core frequency: {NvUtil.current_core_clock} MHz </p>
              <p> Memory frequency: {NvUtil.current_memory_clock} MHz </p>
              <p> Memory usage: {NvUtil.memory_usage}% ({NvUtil.memory_used/1024/1024} MB)</p>
              <p> Temperature: {NvUtil.temperature} °C </p>
            </div>
            <div>
              <h3> Specyfication </h3>
              <p> Cores: {NvSpec.cores} </p>
              <p> Frame buffer: {NvSpec.memory / 1024 / 1024} MB </p>
              <p> Memory bus: {NvSpec.memory_bus} </p>
              <p> PCIe gen: {NvSpec.pci_e_gen} </p>
              <p> PCIe width: {NvSpec.pci_e_width} </p>
              <p> Cuda: {NvSpec.cuda.major}.{NvSpec.cuda.minor} </p>
              <p> Architectur: {NvSpec.arc} </p>
            </div>
          </div>
        </div>
      )   

}