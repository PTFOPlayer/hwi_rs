import { NvStats } from "../../scripts/interfaces";

export default function Nvidia(nvData:{nvidia: NvStats}) {
    let NvSpec = nvData.nvidia.spec;
    let NvUtil = nvData.nvidia.util;
    let NvManagement = nvData.nvidia.management;
    return (
        <div className="cpu">
          <h1> {NvSpec.name} </h1>
          <div className="data">
            <div>
              <h3> General </h3>
              <p> Usage: {NvUtil.core_usage} % </p>
              <p> Core frequency: {NvUtil.current_core_clock} MHz </p>
              <p> Memory frequency: {NvUtil.current_memory_clock} MHz </p>
              <p> Memory usage: {NvUtil.memory_usage}% ({NvUtil.memory_used/1024/1024} MB)</p>
              <p> Temperature: {NvUtil.temperature} °C </p>
              <p> Power usage: {NvUtil.power_usage ? NvUtil.power_usage.toString() : "not supported"} </p>
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
            <div>
              <h3> Management </h3>
              <p> Power limit: {NvManagement.power_limit ? NvManagement.power_limit.toString() : "not supported"} </p>
              <p> Target app core frequency: {NvManagement.target_core_clock ? NvManagement.target_core_clock: "not supported"}</p>
              <p> Target app core frequency: {NvManagement.target_memory_clock ? NvManagement.target_memory_clock: "not supported"}</p>
              <p> Default app core frequency: {NvManagement.default_core_clock ? NvManagement.default_core_clock: "not supported"}</p>
              <p> Default app core frequency: {NvManagement.default_memory_clock ? NvManagement.default_memory_clock: "not supported"}</p>
              <p> App core frequency: {NvManagement.app_core_clock ? NvManagement.app_core_clock: "not supported"}</p>
              <p> App core frequency: {NvManagement.app_memory_clock ? NvManagement.app_memory_clock: "not supported"}</p>
            </div>
          </div>
        </div>
      )   

}