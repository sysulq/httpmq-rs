httpmq-rs
===

[![Rust](https://github.com/hnlq715/httpmq-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hnlq715/httpmq-rs/actions/workflows/rust.yml)

Benchmark
---

Also, you can refer to [github.com/hnlq715/httpmq#benchmark](https://github.com/hnlq715/httpmq#benchmark) for the benchmark of **Golang** implementation.

Test machine(Hackintosh):

```text
                    'c.          
                 ,xNMM.          ----------------------- 
               .OMMMMo           OS: macOS 11.6.1 20G224 x86_64 
               OMMM0,            Host: Hackintosh (SMBIOS: iMac20,1) 
     .;loddo:' loolloddol;.      Kernel: 20.6.0 
   cKMMMMMMMMMMNWMMMMMMMMMM0:    Uptime: 13 hours, 16 mins 
 .KMMMMMMMMMMMMMMMMMMMMMMMWd.    Packages: 45 (brew) 
 XMMMMMMMMMMMMMMMMMMMMMMMX.      Shell: zsh 5.8 
;MMMMMMMMMMMMMMMMMMMMMMMM:       Resolution: 1920x1080@2x 
:MMMMMMMMMMMMMMMMMMMMMMMM:       DE: Aqua 
.MMMMMMMMMMMMMMMMMMMMMMMMX.      WM: Quartz Compositor 
 kMMMMMMMMMMMMMMMMMMMMMMMMWd.    WM Theme: Blue (Dark) 
 .XMMMMMMMMMMMMMMMMMMMMMMMMMMk   Terminal: vscode 
  .XMMMMMMMMMMMMMMMMMMMMMMMMK.   CPU: Intel i5-10600K (12) @ 4.10GHz 
    kMMMMMMMMMMMMMMMMMMMMMMd     GPU: Radeon Pro W5500X 
     ;KMMMMMMMWXXWMMMMMMMk.      Memory: 17549MiB / 32768MiB 
       .cooc,.    .,coo:.
```

PUT

```bash
wrk -c 10 -t 2 -d 10s "http://127.0.0.1:1218/?name=xoyo&opt=put&data=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
Running 10s test @ http://127.0.0.1:1218/?name=xoyo&opt=put&data=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   139.45us  150.62us   9.51ms   99.34%
    Req/Sec    36.01k     1.56k   39.42k    71.78%
  723434 requests in 10.10s, 89.69MB read
Requests/sec:  71629.07
Transfer/sec:      8.88MB
```

GET

```bash
wrk -c 10 -t 2 -d 10s "http://127.0.0.1:1218/?name=xoyo&opt=get"
Running 10s test @ http://127.0.0.1:1218/?name=xoyo&opt=get
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   155.09us  407.65us  12.22ms   99.30%
    Req/Sec    36.70k     3.08k   40.48k    77.72%
  737643 requests in 10.10s, 456.55MB read
Requests/sec:  73034.96
Transfer/sec:     45.20MB
```

Flamegraph
---

PUT

![flamegraph_put](./flamegraph_put.svg)

GET

![flamegraph_get](./flamegraph_get.svg)