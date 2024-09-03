from pydantic import BaseModel # type: ignore
from typing import List


class LogProcess(BaseModel):
    pid: int
    name: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float
    cmdline: str


class LogSystem(BaseModel):
    total_ram: int
    used_ram: int
    free_ram: int
    processes: List[LogProcess]
    timestamp: str
