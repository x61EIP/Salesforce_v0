
import asyncio
from pysnmp.hlapi.v3arch.asyncio import *

async def run():
    snmpEngine = SnmpEngine()

    iterator = get_cmd(
        snmpEngine,
        CommunityData("public", mpModel=0),
        await UdpTransportTarget.create(("
    )
