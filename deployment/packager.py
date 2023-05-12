import hashlib
import sys
from string import Template

args = sys.argv
version = args[1]
template_file_path = args[2]
generated_file_path = args[3]

# SHA512, SHA256, or SHA1
hash_type = args[4]

# Deployment files
deployment_file_path_1 = args[5]
deployment_file_path_2 = args[6] if len(args) > 6 else None
deployment_file_path_3 = args[7] if len(args) > 7 else None

print(f"Generating package for file: {deployment_file_path_1}")
if deployment_file_path_2 is not None:
    print(f"and for file: {deployment_file_path_2}")
if deployment_file_path_3 is not None:
    print(f"and for file: {deployment_file_path_3}")
print(f"     VERSION: {version}")
print(f"     TEMPLATE PATH: {template_file_path}")
print(f"     SAVING AT: {generated_file_path}")
print(f"     USING HASH TYPE: {hash_type}")


def get_hash(deployment_file):
    if str.lower(hash_type) == "sha512":
        deployment_hash = hashlib.sha512(deployment_file.read()).hexdigest()
    elif str.lower(hash_type) == "sha256":
        deployment_hash = hashlib.sha256(deployment_file.read()).hexdigest()
    elif str.lower(hash_type) == "sha1":
        deployment_hash = hashlib.sha1(deployment_file.read()).hexdigest()
    else:
        print('Unsupported hash format "%s".  Please use SHA512, SHA256, or SHA1.', hash_type)
        exit(1)

    print(f"Generated hash: {str(deployment_hash)}")
    return deployment_hash


with open(deployment_file_path_1, "rb") as deployment_file_1:
    deployment_hash_1 = get_hash(deployment_file_1)

    deployment_hash_2 = None
    if deployment_file_path_2 is not None:
        with open(deployment_file_path_2, "rb") as deployment_file_2:
            deployment_hash_2 = get_hash(deployment_file_2)

    deployment_hash_3 = None
    if deployment_file_path_3 is not None:
        with open(deployment_file_path_3, "rb") as deployment_file_3:
            deployment_hash_3 = get_hash(deployment_file_3)

    with open(template_file_path, "r") as template_file:
        template = Template(template_file.read())

        substitutes = {"version": version, "hash1": deployment_hash_1}
        if deployment_hash_2 is not None:
            substitutes["hash2"] = deployment_hash_2
        if deployment_hash_3 is not None:
            substitutes["hash3"] = deployment_hash_3

        substitute = template.safe_substitute(substitutes)

        print("\n================== Generated package file ==================\n")
        print(substitute)
        print("\n============================================================\n")

        with open(generated_file_path, "w") as generated_file:
            generated_file.write(substitute)
