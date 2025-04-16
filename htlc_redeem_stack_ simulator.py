import hashlib

# --- Initial State ---
stack = [
    "4f37cd4f6bea226e7170925e491f78ff677e9e8501fb0b9eb37ac8caf2f63a5200ffffffff", #encoded leaf
    "225120527b67c93c5b95dbf894d1e4c8dccce88bdcb83684e6ad65d32d48880c6b1924ffffffff", #pervout scriptpubkey + input sequencer
    "a086010000000000", # input and output amount
    "6fa68ff638a1b376512842830368648d7ea1fbcf9b7b3c5a071308393c619cc400000000", #pervouts
    "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817987817a3d3732d96e61fcd8a6bad6133104d0e1f8ea3649c2d69ac3b446e3203",
    "be", #last bit sign
    "bf", #last bit sign + 1
    "6e0d8625db81003c347c5ccc2c26f3a6b3cc8991c05624b9eeb80b1357ca8408", #Preimage
]


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
script_string = "OP_SHA256 OP_PUSHBYTES_32 6c93e898c1bcf964c54bbdc8bafeb5ab557ccba4f7f7a1f55cecb80581875d9f OP_EQUALVERIFY OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK OP_PUSHBYTES_12 0083020000000000000002 OP_SWAP OP_CAT OP_SWAP OP_DUP OP_TOALTSTACK OP_CAT OP_SWAP OP_CAT OP_FROMALTSTACK OP_PUSHBYTES_32 225120527b67c93c5b95dbf894d1e4c8dccce88bdcb83684e6ad65d32d48880c6b1924 OP_CAT OP_SHA256 OP_CAT OP_SWAP OP_CAT OP_PUSHBYTES_10 54617053696768617368 OP_SHA256 OP_DUP OP_ROT OP_CAT OP_CAT OP_SHA256 OP_PUSHBYTES_20 424950303334302f6368616c6c656e6765 OP_SHA256 OP_DUP OP_ROT OP_PUSHBYTES_32 79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798 OP_DUP OP_DUP OP_DUP OP_TOALTSTACK OP_TOALTSTACK OP_ROT OP_CAT OP_CAT OP_CAT OP_CAT OP_SHA256 OP_FROMALTSTACK OP_SWAP OP_CAT OP_FROMALTSTACK OP_FROMALTSTACK OP_ROT OP_SWAP OP_DUP OP_FROMALTSTACK OP_CAT OP_ROT OP_EQUALVERIFY OP_FROMALTSTACK OP_CAT OP_SWAP OP_CHECKSIG"
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