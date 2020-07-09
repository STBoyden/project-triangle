#!/bin/python3

import sys
import os
import shutil
import subprocess
from pathlib import Path

dir_path = os.getcwd()
dir_name = dir_path.split("/")[-1].replace("-", "_") if os.name == "posix" else dir_path.split("\\")[-1].replace("-", "_")
copy_directories = ["assets", "maps"]
distribution_dir = "distrib"

commands = ' '.join([x for x in sys.argv[1:]])

result = subprocess.call(["cargo"] + sys.argv[1:])

if result:
    if os.path.exists(Path(f"{dir_path}/target/debug")):
        for directory in copy_directories:
                shutil.copytree(Path(f"{dir_path}/{directory}"), \
                            Path(f"{dir_path}/target/debug/{directory}"))

if not os.path.exists(Path(f"{dir_path}/{distribution_dir}")):
    os.makedirs(Path(f"{dir_path}/{distribution_dir}"))

print("")
print(f"--- [ Copying {'Linux' if os.name == 'posix' else 'Windows'} files ] --- ")
for directory in copy_directories:
    shutil.copytree(Path(f"{dir_path}/{directory}"), Path(f"{dir_path}/{distribution_dir}/{directory}"))
shutil.copyfile(Path(f"{dir_path}/target/debug/{dir_name}{'' if os.name == 'posix' else '.exe'}"), \
                Path(f"{dir_path}/{distribution_dir}/{dir_name}{'' if os.name == 'posix' else '.exe'}"))

print("Done")
