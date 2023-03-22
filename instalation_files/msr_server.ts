import express from "express";
import { exec, ExecException } from 'child_process';
//const express = require("express")

const app = express();
const port = 3230;
app.use(express.json())

type Res = {
    write: (arg0: string) => void, 
    send: (arg0: string | null | ExecException | void) => void
}

interface msr_obj {
    cpu: {
        vendor: string,
        name: string,
        power: number,
        voltage: number,
        usage: number,
        temperature: number,
        hyper_threading: number,
        logical_cores: number,
        physical_cores: number,
    }
    memory: {
        total: number
        available: number
        used: number
    }
}

interface exec_json {
    key: string,
    command: string
}

app.get('/', (_req, res: Res) => {
    exec('sudo msr_gen -o', (error: ExecException | null, stdout: string, stderr: string) => {
        if (stdout !== null)
            return res.send(stdout);
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    });
});

app.get('/json/*', (req: {url: string}, res: Res) => {
    exec('sudo msr_gen -j', (error: ExecException | null, stdout: string, stderr: string) => {
        let obj: msr_obj = JSON.parse(stdout)
        if (stdout !== null) {
            if (req.url === "/json" || req.url === "/json/") res.write(stdout);
            else if (req.url.match('/json/cpu\*') ){
                if (req.url.includes('/vendor')) res.write(obj.cpu.vendor.toString()+" ");
                if (req.url.includes('/name')) res.write(obj.cpu.name.toString()+" ");
                if (req.url.includes('/power')) res.write(obj.cpu.power.toString()+" ");
                if (req.url.includes('/voltage')) res.write(obj.cpu.voltage.toString()+" ");
                if (req.url.includes('/usage')) res.write(obj.cpu.usage.toString()+" ");
                if (req.url.includes('/temperature')) res.write(obj.cpu.temperature.toString()+" ");
            }
            else if (req.url.match('/json/memory\*')) {
                if (req.url.includes('/total')) res.write(obj.memory.total.toString()+" ");
                if (req.url.includes('/available')) res.write(obj.memory.available.toString()+" ");
                if (req.url.includes('/used')) res.write(obj.memory.used.toString()+" ");
            }
            return res.send()
        }
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    });
});

const execute_middleware =(req: { body: string; }): string => {
    let json: exec_json = JSON.parse(JSON.stringify(req.body))
    if (json.key !== null && process.env.MSR_KEY !== null) {
        if (json.command !== null && process.env.MSR_KEY === json.key)
            return json.command
        else {
            return "echo \"no command supplied in attached json or wrong key\""
        }
    } else {
        return "echo \"no key supplied in attached json or system variable\""
    }
}

app.post('/execute', (req: { body: string }, res: Res) => {
    exec(execute_middleware(req), (error, stdout, stderr) => {
        if (stdout !== null)
            return res.send(stdout);
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    })
})

app.listen(port, () => console.log(`listening on port ${port}`));