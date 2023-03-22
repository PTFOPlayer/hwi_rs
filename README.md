<h1 align="center"> hwi_rs </h1>
<p>
  <a href="#" target="_blank">
    <img alt="License: GNU GPL 3.0" src="https://img.shields.io/badge/License-GNU GPL 3.0-yellow.svg" />
  </a>
</p>

## hwi_rs is a program displays CPU and GPU data to user in compact and understandable way. 

# instalation
to install basic version you don't need any dependencies just do all the steps.

first clone repository
```
    git clone https://github.com/PTFOPlayer/hwi_rs && cd hwi_rs
```

then run instalation script
```
    ./install.sh
```

you can also run `./install.sh -s` to install additional dependencies that allow you to make requests via api, you will need to install:
  * nodejs
  * npm

usage of api will be explained in msr_server ( currently is not )

# build from source
to build from source you need several packages:
  * cargo (rustup)
  * cmake
  * make
  * g++
  * nodejs
  * npm

first clone repository
```
    git clone https://github.com/PTFOPlayer/hwi_rs && cdc hwi_rs
```

then run build
```
    ./build.sh
```