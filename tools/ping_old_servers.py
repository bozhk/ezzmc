#!/usr/bin/env python3
"""Протокол старых версий первоисточником (TODO-459): запускает ванильный server.jar Mojang и пингует его.
1.6.x — legacy-пинг FE 01 FA MC|PingHost (ответ «§1\0protocol\0version…»), 1.7+ — Status JSON.
Использование: tools/ping_old_servers.py <каталог с <ver>.jar> <ver>...  (нужна Java 8: $JAVA8)."""
import socket, struct, json, sys, subprocess, time, os
J8=os.environ.get("JAVA8","/usr/lib/jvm/java-8-openjdk/bin/java")
base=sys.argv[1]
def varint(n):
    out=b""
    while True:
        b=n&0x7f; n>>=7
        if n: out+=bytes([b|0x80])
        else: return out+bytes([b])
def rvarint(s):
    n=0;sh=0
    while True:
        b=s.recv(1)[0]; n|=(b&0x7f)<<sh; sh+=7
        if not b&0x80: return n
def recvn(s,n):
    d=b""
    while len(d)<n:
        c=s.recv(n-len(d))
        if not c: raise EOFError
        d+=c
    return d
def ping_netty(port):
    s=socket.create_connection(("127.0.0.1",port),5)
    host=b"127.0.0.1"
    hs=varint(0)+varint(4)+varint(len(host))+host+struct.pack(">H",port)+varint(1)
    s.sendall(varint(len(hs))+hs); s.sendall(varint(1)+varint(0))
    rvarint(s); rvarint(s); n=rvarint(s)
    return json.loads(recvn(s,n))["version"]
def ping_legacy(port):
    s=socket.create_connection(("127.0.0.1",port),5)
    s16=lambda t: struct.pack(">H",len(t))+t.encode("utf-16-be")
    data=bytes([73])+s16("127.0.0.1")+struct.pack(">i",port)
    s.sendall(b"\xfe\x01\xfa"+s16("MC|PingHost")+struct.pack(">H",len(data))+data)
    assert recvn(s,1)==b"\xff"
    n=struct.unpack(">H",recvn(s,2))[0]
    t=recvn(s,n*2).decode("utf-16-be").split("\x00")
    return {"protocol":int(t[1]),"name":t[2]}
for i,v in enumerate(sys.argv[2:]):
    d=f"{base}/run-{v}"; os.makedirs(d,exist_ok=True)
    port=25700+i
    open(f"{d}/eula.txt","w").write("eula=true\n")
    open(f"{d}/server.properties","w").write(f"server-port={port}\nonline-mode=false\nlevel-type=FLAT\ngenerate-structures=false\nspawn-animals=false\nspawn-monsters=false\n")
    p=subprocess.Popen([J8,"-Xmx512M","-jar",f"{base}/{v}.jar","nogui"],cwd=d,stdin=subprocess.PIPE,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    res=None
    for _ in range(90):
        time.sleep(1)
        try:
            res=(ping_legacy if v.startswith("1.6") else ping_netty)(port); break
        except Exception as e: err=e
    print(v, res if res else f"FAIL {err}", flush=True)
    try: p.stdin.write(b"stop\n"); p.stdin.flush(); p.wait(30)
    except Exception: p.kill()
