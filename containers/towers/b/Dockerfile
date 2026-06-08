from python:3.13
workdir /towers

copy towers/requirements.txt ./
run pip install --no-cache-dir -r requirements.txt

copy towers/b/data ./data
expose 10161/udp

run useradd towercontroller
user towercontroller

cmd ["snmpsim-command-responder", "--data-dir=./data","--agent-udpv4-endpoint=0.0.0.0:10161"]
