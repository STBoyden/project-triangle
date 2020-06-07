#!/bin/python3

import sys
import os
import shutil
import subprocess
from pathlib import Path

dir_path = os.getcwd()
dir_name = dir_path.split("/")[-1].replace("-", "_") if os.name == "posix" else dir_path.split("\\")[-1].replace("-", "_")
resource_directory = "assets"
distribution_dir = "distrib"

commands = ' '.join([x for x in sys.argv[1:]])

result = subprocess.call(["cargo"] + sys.argv[1:])

if result:
    if os.path.exists(Path(f"{dir_path}/target/release")):
        shutil.copytree(Path(f"{dir_path}/{resource_directory}"), \
                        Path(f"{dir_path}/target/release/{resource_directory}"))

if not os.path.exists(Path(f"{dir_path}/{distribution_dir}")):
    os.makedirs(Path(f"{dir_path}/{distribution_dir}"))

print("")
print(f"--- [ Copying {'Linux' if os.name == 'posix' else 'Windows'} files ] --- ")
if os.name == "posix":
    subprocess.call(["strip", "--strip-all", "target/release/project_triangle"])
shutil.copytree(Path(f"{dir_path}/{resource_directory}"), Path(f"{dir_path}/{distribution_dir}/{resource_directory}"))
shutil.copyfile(Path(f"{dir_path}/target/release/{dir_name}{'' if os.name == 'posix' else '.exe'}"), \
                Path(f"{dir_path}/{distribution_dir}/{dir_name}{'' if os.name == 'posix' else '.exe'}"))

print("Done")
