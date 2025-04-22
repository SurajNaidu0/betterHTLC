import hashlib

hex_script = "51206654f131d187d0ae56783b4d7c8aa1dee6ac5e0273a98e196866ca349852890d"
script_bytes = bytes.fromhex(hex_script)
digest = hashlib.sha256(script_bytes).hexdigest()
print(digest)
