//import express from "express";
import { exec } from 'child_process';
const express = require("express")

const app = express();
const port = 3230;

app.get('/', async (req:any, res: { send: (arg0: any) => void; }) => {
    await exec('sudo msr_gen -o', (error, stdout, stderr) => {
        if( stdout !== null) return res.send(stdout);
        else if ( stderr !== null) res.send(stderr)
        else return res.send(error);
    });
});

app.listen(port, () => console.log(`listening on port ${port}`));