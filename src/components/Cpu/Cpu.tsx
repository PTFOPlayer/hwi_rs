import { Msr } from "../../scripts/interfaces";
import "./cpu.scss"
export default function Cpu(data: { msr:Msr  }) {
  let msr = data.msr;
  return (
    <div className="cpu">
      <h1> {msr.name ? msr.name : null} </h1>
      <div className="data">
        <div>
          <h3> General </h3>
          <p> Usage: {msr.util.toPrecision(5)} % </p>
          <p> Avarage frequency: {msr.freq.toPrecision(5)} MHz </p>
          <p> Power: {msr.package_power.toPrecision(5)} W </p>
          <p> Voltage: {msr.voltage.toPrecision(5)} V </p>
          <p> Temperature: {msr.temperature.toPrecision(5)} °C </p>
        </div>
        <div>
          <h3> Specyfication </h3>
          <p> Cores: {msr.cores} </p>
          <p> Threads: {msr.threads} </p>
          {msr.cache ? msr.cache.map((e) => {
            let size = e.size / 1024
            return (<p>Cache L{e.level} {e.cache_type}: {e.size / 1024 > 1024 ? <span>{size / 1024} MB </span> : <span>{size} KB </span>}</p>)
          }) : null}
        </div>
        <div>
          <h3> Per core frequency </h3>
          {msr.per_core_freq.map((e, key) => {
            return <><p>Core: {key}: {e.toPrecision(5)}</p></>
          })}
        </div>
        <div>
          <h3> Memory </h3>
          <p>Total: {msr.mem_total}</p>
          <p>Used: {msr.mem_used}</p>
          <p>Available: {msr.mem_free}</p>
        </div>
      </div>
    </div>
  )

}