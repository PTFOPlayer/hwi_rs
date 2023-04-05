import { CpuData } from "../../scripts/interfaces";
import "./cpu.scss"
export default function Cpu(cpuData: { cpu: CpuData }) {
  let cpu = cpuData.cpu

  let avg_frequency = () => {
    let len = cpu.frequency.length;
    let sum = 0
    for (let i = 0; i < len; i++) {
      sum += cpu.frequency[i]
    };
    return sum / len
  }

  return (
    <div className="cpu">
      <h1> CPU: {cpu.name ? cpu.name : null} </h1>
      <div className="data">
        <div>
          <h3> General </h3>
          <p> Usage: {cpu.load.toPrecision(5)} % </p>
          <p> Avarage frequency: {avg_frequency().toPrecision(5)} MHz </p>
          <p> Power: {cpu.power.toPrecision(5)} W </p>
          <p> Voltage: {cpu.voltage.toPrecision(5)} V </p>
          <p> Temperature: {cpu.temperature} °C </p>
        </div>
        <div>
          <h3> Specyfication </h3>
          <p> Cores: {cpu.physical_cores} </p>
          <p> Threads: {cpu.logical_cores} </p>
        </div>
        <div>
          <h3> Cache info </h3>
          {cpu.cache ? cpu.cache.map((e) => {
            let size = e.size/1024
            return (<p>L{e.level} {e.cache_type}: {e.size/1024 > 1024 ? <span>{size/1024} MB </span> : <span>{size} KB </span> }</p>)
          }) : null}
        </div>
        <div>
          <h3> Per core frequency </h3>
          {cpu.frequency.map((e,key )=> {
            return <><p>{`Core:${key}: ${e.toPrecision(5)}`}</p></>
          })}
        </div>
      </div>
    </div>
  )

}