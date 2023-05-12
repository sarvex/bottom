# Because choco is a special case and I'm too lazy to make my
# packaging script robust enough, so whatever, hard-code time.

import hashlib
import sys
from string import Template
import os

args = sys.argv
deployment_file_path_32 = args[1]
deployment_file_path_64 = args[2]
version = args[3]
nuspec_template = args[4]
ps1_template = args[5]
generated_nuspec = args[6]
generated_ps1 = args[7]
generated_ps1_dir = args[8]

print("Generating Chocolatey package for:")
print(f"     32-bit: {deployment_file_path_32}")
print(f"     64-bit: {deployment_file_path_64}")
print(f"     VERSION: {version}")
print(f"     NUSPEC TEMPLATE: {nuspec_template}")
print(f"     PS1 TEMPLATE: {ps1_template}")
print(f"     GENERATED NUSPEC: {generated_nuspec}")
print(f"     GENERATED PS1: {generated_ps1}")
print(f"     GENERATED PS1 DIR: {generated_ps1_dir}")

with (open(deployment_file_path_32, "rb") as deployment_file_32, open(
    deployment_file_path_64, "rb"
) as deployment_file_64):
    hash_32 = hashlib.sha1(deployment_file_32.read()).hexdigest()
    hash_64 = hashlib.sha1(deployment_file_64.read()).hexdigest()

    print(f"Generated 32 hash: {str(hash_32)}")
    print(f"Generated 64 hash: {str(hash_64)}")

    with open(nuspec_template, "r") as template_file:
        template = Template(template_file.read())
        substitute = template.safe_substitute(version=version)
        print("\n================== Generated nuspec file ==================\n")
        print(substitute)
        print("\n============================================================\n")

        with open(generated_nuspec, "w") as generated_file:
            generated_file.write(substitute)

    os.makedirs(generated_ps1_dir)
    with open(ps1_template, "r") as template_file:
        template = Template(template_file.read())
        substitute = template.safe_substitute(version=version, hash_32=hash_32, hash_64=hash_64)
        print("\n================== Generated chocolateyinstall file ==================\n")
        print(substitute)
        print("\n============================================================\n")

        with open(generated_ps1, "w") as generated_file:
            generated_file.write(substitute)
