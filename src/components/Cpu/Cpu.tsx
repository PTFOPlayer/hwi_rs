import { Msr } from "../../scripts/interfaces";
import "./cpu.scss"
export default function Cpu(data: { msr:Msr  }) {
  let cpu = data.msr.cpu
  let memory = data.msr.memory

  return (
    <div className="cpu">
      <h1> {cpu.name ? cpu.name : null} </h1>
      <div className="data">
        <div>
          <h3> General </h3>
          <p> Usage: {cpu.util.toPrecision(5)} % </p>
          <p> Avarage frequency: {cpu.freq.toPrecision(5)} MHz </p>
          <p> Power: {cpu.package_power.toPrecision(5)} W </p>
          <p> Voltage: {cpu.voltage.toPrecision(5)} V </p>
          <p> Temperature: {cpu.temperature.toPrecision(5)} °C </p>
        </div>
        <div>
          <h3> Specyfication </h3>
          <p> Cores: {cpu.cores} </p>
          <p> Threads: {cpu.threads} </p>
          {cpu.cache ? cpu.cache.map((e) => {
            let size = e.size / 1024
            return (<p>Cache L{e.level} {e.cache_type}: {e.size / 1024 > 1024 ? <span>{size / 1024} MB </span> : <span>{size} KB </span>}</p>)
          }) : null}
        </div>
        <div>
          <h3> Per core frequency </h3>
          {cpu.per_core_freq.map((e, key) => {
            return <><p>Core: {key}: {e.toPrecision(5)}</p></>
          })}
        </div>
        <div>
          <h3> Memory </h3>
          <p>Total: {memory.mem_total}</p>
          <p>Used: {memory.mem_used}</p>
          <p>Available: {memory.mem_free}</p>
        </div>
      </div>
    </div>
  )

}