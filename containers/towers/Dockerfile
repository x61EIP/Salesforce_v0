from python:3.13
workdir /towers

copy requirements.txt ./
run pip install --no-cache-dir -r requirements.txt

copy data ./data
expose 9090

run useradd towercontroller
user towercontroller

cmd ["snmpsim-command-responder", "--data-dir=./data", "--agent-udpv4-endpoint=0.0.0.0:9090"]
