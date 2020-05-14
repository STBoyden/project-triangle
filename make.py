#!/bin/python3

import sys
import os
import shutil
import subprocess

dir_path = os.getcwd()
dir_name = dir_path.split("/")[-1].replace("-", "_")
resource_directory = "assets"
distribution_dir = "distrib"

#if os.path.exists(f"{dir_path}/target/debug"):
#    print("path found\n")
#    shutil.rmtree(f"{dir_path}/target/debug/{resource_directory}")

commands = ' '.join([x for x in sys.argv[1:]])

result = subprocess.call(["cargo"] + sys.argv[1:])

if result:
    if os.path.exists(f"{dir_path}/target/debug"):
        shutil.copytree(f"{dir_path}/{resource_directory}", 
                        f"{dir_path}/target/debug/{resource_directory}")

if not os.path.exists(f"{dir_path}/{distribution_dir}"):
    os.makedirs(f"{dir_path}/{distribution_dir}")

print("")
print(f"--- [ Copying {'Linux' if os.name == 'posix' else 'Windows'} files ] --- ")
shutil.copytree(f"{dir_path}/{resource_directory}", f"{dir_path}/{distribution_dir}/{resource_directory}")
shutil.copyfile(f"{dir_path}/target/debug/{dir_name}{'' if os.name == 'posix' else '.exe'}", \
                f"{dir_path}/{distribution_dir}/{dir_name}{'' if os.name == 'posix' else '.exe'}")
