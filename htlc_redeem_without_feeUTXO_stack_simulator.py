import hashlib
['00',#epoch
 '00', 
 '02000000', #vesion
 '00000000', #locktime
 '89b0d51bc691fe7f759a848d636ae6134a1c867cbc9ed7b391baa9068494ebba', #Prevoutpoint hashed
 '184c0ede118ec8cd31f699524bae48a81ed01ded5f8d08f2f4ff4286b33b0270', #Prev Input amount hashed
 'a4948faaa007be758a8c2a380a2beaf2cd807e5d54bae2a4e7710f221c66397a', #Prev Input scriptpubkey hashed
 'ad95131bc0b799c0b1af477fb14fcf26a6a9f76079e48bf090acb7e8367bfd0e', #Hashed Sequencer
 'bbe0063d023f50ff192a2b334d4cefabbe126a94202abb906afa80178332f090', #Hashed amount+output
 '02', 
 '00000000', 
 '7721e57eea76a99c7e60f94cd2dfdf1e518ecb92b7d9738745b5ade60adb4252', #leaf
 '00', 
 'ffffffff']

[
    '00', # witness version
    '00', #
    '02000000', #vesion
    '00000000', #locktime
    '89b0d51bc691fe7f759a848d636ae6134a1c867cbc9ed7b391baa9068494ebba',
    'a086010000000000', #previous amount
    'a4948faaa007be758a8c2a380a2beaf2cd807e5d54bae2a4e7710f221c66397a',
    'ad95131bc0b799c0b1af477fb14fcf26a6a9f76079e48bf090acb7e8367bfd0e',
    'b882010000000000',#amount to send 
    '225120527b67c93c5b95dbf894d1e4c8dccce88bdcb83684e6ad65d32d48880c6b1924'#redeem address (should be in the script),
    '02', 
    '00000000', 
    '7721e57eea76a99c7e60f94cd2dfdf1e518ecb92b7d9738745b5ade60adb4252', #leaf
    '00', 
    'ffffffff']


# --- Initial State ---
stack = [
    '00', # witness version
    '00', #
    '02000000', #vesion
    '00000000', #locktime
    '89b0d51bc691fe7f759a848d636ae6134a1c867cbc9ed7b391baa9068494ebba',
    '184c0ede118ec8cd31f699524bae48a81ed01ded5f8d08f2f4ff4286b33b0270', #previous amount
    'a4948faaa007be758a8c2a380a2beaf2cd807e5d54bae2a4e7710f221c66397a',
    'ad95131bc0b799c0b1af477fb14fcf26a6a9f76079e48bf090acb7e8367bfd0e',
    'b882010000000000',#amount to send 
    '02', 
    '00000000', 
    '7721e57eea76a99c7e60f94cd2dfdf1e518ecb92b7d9738745b5ade60adb4252', #leaf
    '00', 
    'ffffffff',
    '79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798224fde4ccb32c50a04c39c5d00134174fd69d3c5a9e8b5d88ba983ac096ac2',
    'ab',
    'ac',
    '6644fd23b8327a04d86bdadbeba6903c1e9bfef68f9c9ee7c00cc8f59529430c']
#'225120527b67c93c5b95dbf894d1e4c8dccce88bdcb83684e6ad65d32d48880c6b1924',#redeem address (should be in the script),

alt_stack = []

# --- Helper Functions ---
def sha256_hex(hex_string):
    """Computes SHA256 hash of a hex string and returns hex digest."""
    if not hex_string: # Handle empty strings if they occur
         return hashlib.sha256(b'').hexdigest()
    try:
        return hashlib.sha256(bytes.fromhex(hex_string)).hexdigest()
    except ValueError as e:
        print(f"Error hashing hex string: {hex_string} - {e}")
        raise # Re-raise the error to stop execution if invalid hex

def debug_stack(msg=""):
    """Prints the current state of both stacks."""
    print(f"\n--- {msg} ---")
    # Print stack top-down for better readability
    print(f"Stack ({len(stack)} items, top first): {list(reversed(stack))}")
    print(f"AltStack ({len(alt_stack)} items, top first): {list(reversed(alt_stack))}")
    print("-" * (len(msg) + 6))

# --- Script Definition ---

script_string = "OP_SHA256 OP_PUSHBYTES_32 7d71c056feba9afeb8ee135b8c83695b1ecf948a96d24494592a5743c6779a57 OP_EQUALVERIFY OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK OP_CAT OP_CAT OP_CAT OP_CAT OP_SWAP OP_PUSHBYTES_32 225120527b67c93c5b95dbf894d1e4c8dccce88bdcb83684e6ad65d32d48880c6b1924 OP_CAT OP_SHA256 OP_SWAP OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_PUSHBYTES_10 54617053696768617368 OP_SHA256 OP_DUP OP_ROT OP_CAT OP_CAT OP_SHA256 OP_PUSHBYTES_17 424950303334302f6368616c6c656e6765 OP_SHA256 OP_DUP OP_ROT OP_PUSHBYTES_32 79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798 OP_DUP OP_DUP OP_DUP OP_TOALTSTACK OP_TOALTSTACK OP_ROT OP_CAT OP_CAT OP_CAT OP_CAT OP_SHA256 OP_FROMALTSTACK OP_SWAP OP_CAT OP_FROMALTSTACK OP_FROMALTSTACK OP_ROT OP_SWAP OP_DUP OP_FROMALTSTACK OP_CAT OP_ROT OP_EQUALVERIFY OP_FROMALTSTACK OP_CAT OP_SWAP OP_CHECKSIG" 
script_ops = script_string.split(' ')

# --- Script Execution Simulation ---

print("--- Initial State ---")
debug_stack("Initial Stacks")




pc = 0 # Program counter
while pc < len(script_ops):
    op = script_ops[pc]
    operation_description = f"Executing: {op}"
    pc += 1 # Increment pc assume 1 element unless it's a push

    try:
        if op.startswith("OP_PUSHBYTES_"):
            if pc >= len(script_ops): raise IndexError(f"Missing data for {op}")
            data = script_ops[pc]
            pc += 1 # Increment pc again for the data
            stack.append(data)
            operation_description = f"Executing: {op} {data}"

        elif op == "OP_CHECKSIGVERIFY":
             # Simulates success: Pop public key and (implicitly popped earlier) signature
            if len(stack) < 1: raise IndexError("Stack underflow for OP_CHECKSIGVERIFY (pubkey)")
            stack.pop() # Pop public key

        elif op == "OP_TOALTSTACK":
            if not stack: raise IndexError("Stack underflow for OP_TOALTSTACK")
            alt_stack.append(stack.pop())

        elif op == "OP_FROMALTSTACK":
            if not alt_stack: raise IndexError("AltStack underflow for OP_FROMALTSTACK")
            stack.append(alt_stack.pop())

        elif op == "OP_2DUP":
            if len(stack) < 2: raise IndexError("Stack underflow for OP_2DUP")
            b = stack[-1]
            a = stack[-2]
            stack.append(a)
            stack.append(b)

        elif op == "OP_CAT":
            if len(stack) < 2: raise IndexError("Stack underflow for OP_CAT")
            # Note: OP_CAT is disabled in standard Bitcoin script
            a = stack.pop()
            b = stack.pop()
            stack.append(b + a) # Correct order: second popped + first popped

        elif op == "OP_SWAP":
            if len(stack) < 2: raise IndexError("Stack underflow for OP_SWAP")
            a = stack.pop()
            b = stack.pop()
            stack.append(a)
            stack.append(b)

        elif op == "OP_SHA256":
            if not stack: raise IndexError("Stack underflow for OP_SHA256")
            h = sha256_hex(stack.pop())
            stack.append(h)

        elif op == "OP_DUP":
            if not stack: raise IndexError("Stack underflow for OP_DUP")
            stack.append(stack[-1])

        elif op == "OP_ROT":
            if len(stack) < 3: raise IndexError("Stack underflow for OP_ROT")
            c = stack.pop() # Top
            b = stack.pop() # Second
            a = stack.pop() # Third
            stack.append(b)
            stack.append(c)
            stack.append(a) # Third becomes top

        elif op == "OP_EQUALVERIFY":
             # Simulates success: Pop two items
            if len(stack) < 2: raise IndexError("Stack underflow for OP_EQUALVERIFY")
            first = stack.pop()
            second = stack.pop()
            if first != second:
                print("Failed OP_EQUAL")
                break


        elif op == "OP_CHECKSIG":
            # Simulates success: Pop public key and signature
            # Technically pushes True (01), but for this trace we just consume inputs
            if len(stack) < 2: raise IndexError("Stack underflow for OP_CHECKSIG")
            stack.pop() # Public key
            stack.pop() # Signature
            # We could append "01" here to be more accurate, but often the script ends here
            # or expects the stack to be clean except for a potential final '1' if required.
            # For this visualization, just consuming inputs is clearer.

        else:
            print(f"Unknown OP Code: {op}")
            exit() # Stop execution if unknown opcode

        debug_stack(operation_description)

    except IndexError as e:
        print(f"\n--- SCRIPT FAILED at '{operation_description}' ---")
        print(f"Error: {e}")
        print("Final Stacks before failure:")
        print(f"Stack ({len(stack)} items, top first): {list(reversed(stack))}")
        print(f"AltStack ({len(alt_stack)} items, top first): {list(reversed(alt_stack))}")
        print("-" * 30)
        exit() # Stop execution on failure

# --- Final State ---
print("\n--- Script Execution Finished ---")
print("Final Stacks:")
debug_stack("Final State")

# Optional: Check final stack state (often expects a single 'True' value, represented as '01')
# if len(stack) == 1 and stack[0] == '01':
#     print("\nScript finished successfully with TRUE on stack.")
# elif not stack:
#      print("\nScript finished successfully with empty stack (typical for VERIFY opcodes).")
# else:
#      print("\nScript finished, but final stack state might not be TRUE.")