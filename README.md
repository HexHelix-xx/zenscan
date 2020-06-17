## zenscan

This program scans the open ports of the server.
If your CPU doesn't have enough computing power, it may not work properly.
We are working on fixing the bug.

## How to use

This Program is used to `zenscan < -i IpAddress or --host TargetHost>` format.

## Options
You can be obtained with `zenscan -h`
```
$ zenscan -h

        Zen-Scan 0.1.3
        Nirna Kayanovsky <nirnakayanovsky@gmail.com>

        This program is port scanner to target server.
        not' normal port scanning to do and destructive specification changes may occur.
        This program is test stage.

        USAGE:
            zenscan <options> 
                  
        Options:
        -h, --help            Display this massage
        -V, --version         Display version info
        -i <IPAddress>        The address to scan
        --host <Hostname>     The host scan, trans as IP

```

### output
```
$ zenscan -i 172.17.0.2
[+] start scanning... by 5s. zenscan version for 0.1.3
[+] found the open port => 21
[+] found the open port => 23
[+] found the open port => 22
[+] found the open port => 25
[+] found the open port => 111
[+] found the open port => 139
[+] found the open port => 445
[+] found the open port => 512
[+] found the open port => 513
[+] found the open port => 514
[+] found the open port => 1099
[+] found the open port => 1524
[+] found the open port => 2121
[+] found the open port => 3306
[+] found the open port => 3632
[+] found the open port => 5432
[+] found the open port => 6667
[+] found the open port => 6697
[+] found the open port => 8009
[+] found the open port => 8180
[+] found the open port => 8787
[+] found the open port => 41133
----------------------open ports----------------------
PORTS
21
23
22
25
111
139
445
512
513
514
1099
1524
2121
3306
3632
5432
6667
6697
8009
8180
8787
41133
--------------------------end-------------------------
```
