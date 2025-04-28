import hashlib
import binascii # Useful for potential hex<->bytes conversions if needed later

# --- Initial State ---
# NOTE: Stack is empty initially. For a real transaction,
# this would be populated with witness items BEFORE script execution.
# You need to add the required witness elements here manually
# based on the specific transaction being verified.
stack = [
    "00",  # item 0
    "00",  # item 1
    "02000000",  # item 2
    "00000000",  # item 3
    "84b9aa8c6094f072d701d113601b2bf1994d986bbec340073cdb442ef4ef926b",  # item 4
    "2c3417ea6a6d07f4a0218f5b1d380037c8b95c24eeb4e70299776a634a0dffa2",  # item 5
    "23661260be9b7818aa122a7518395352d885fb578318f2b5daa8c8204a457eb7",  # item 6
    "40e736c02a102a050e1555781b4171020a4279adaa7ed9ca3cc9633a0ade9c37",  # item 7
    "18ddf50500000000",  # item 8
    "02",  # item 9
    "00000000",  # item 10
    "20515c6d7e1f62f1792155d63dfe4a52f38ad8b4149b7644dd6729c52021aa1b",  # item 11
    "00",  # item 12
    "ffffffff",  # item 13
    "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798e5ec592a296f83af8bf15e2b2e8c2ed00c396b5e3aa723c0e7c7f3feef166e", # item 14
    "9d", # item 15
    "9e" # item 16
]

alt_stack = []

# --- Helper Functions ---
def sha256_hex(hex_string):
    """Computes SHA256 hash of a hex string and returns hex digest."""
    if not isinstance(hex_string, str):
        raise TypeError(f"Input must be a hex string, got {type(hex_string)}: {hex_string}")
    if not hex_string: # Handle empty strings
         # In Bitcoin Script, hashing an empty vector results in the hash of an empty byte array
         return hashlib.sha256(b'').hexdigest()
    try:
        # Ensure the hex string has an even number of digits
        if len(hex_string) % 2 != 0:
            print(f"Warning: Odd-length hex string provided to sha256_hex: '{hex_string}'. Prepending '0'.")
            hex_string = '0' + hex_string
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

script_string = "OP_PUSHBYTES_1 64 OP_CHECKSEQUENCEVERIFY OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK OP_CAT OP_CAT OP_CAT OP_CAT OP_SWAP OP_PUSHBYTES_35 225120d0bce2375d5825345cb72c3fbeec62394839f04a13d09e42735cce7dbd6457f3 OP_CAT OP_SHA256 OP_SWAP OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_CAT OP_PUSHBYTES_10 54617053696768617368 OP_SHA256 OP_DUP OP_ROT OP_CAT OP_CAT OP_SHA256 OP_PUSHBYTES_17 424950303334302f6368616c6c656e6765 OP_SHA256 OP_DUP OP_ROT OP_PUSHBYTES_32 79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798 OP_DUP OP_DUP OP_DUP OP_TOALTSTACK OP_TOALTSTACK OP_ROT OP_CAT OP_CAT OP_CAT OP_CAT OP_SHA256 OP_FROMALTSTACK OP_SWAP OP_CAT OP_FROMALTSTACK OP_FROMALTSTACK OP_ROT OP_SWAP OP_DUP OP_FROMALTSTACK OP_CAT OP_ROT OP_EQUALVERIFY OP_FROMALTSTACK OP_CAT OP_SWAP OP_CHECKSIG"
script_ops = script_string.split(' ')

# --- Script Execution Simulation ---

print("--- Initial State ---")
# IMPORTANT: Remember to populate the 'stack' list above with
# the necessary initial witness data before running the simulation.
if not stack:
    print("WARNING: Initial stack is empty. Ensure witness data is added for a valid run.")
debug_stack("Initial Stacks (Before Script Execution)")


pc = 0 # Program counter
while pc < len(script_ops):
    op = script_ops[pc]
    operation_description = f"Executing OP: {op}" # Default description
    pc += 1 # Increment pc, assume 1 element unless it's a push

    try:
        if op.startswith("OP_PUSHBYTES_"):
            n_bytes_str = op.split('_')[-1]
            try:
                # Determine if the length is hex or decimal (Bitcoin uses decimal in names)
                n_bytes = int(n_bytes_str)
            except ValueError:
                 raise ValueError(f"Invalid OP_PUSHBYTES format: {op}")

            if pc >= len(script_ops): raise IndexError(f"Missing data for {op}")
            data = script_ops[pc]
            # Optional: Validate data length matches n_bytes (in bytes, so hex length / 2)
            # expected_hex_len = n_bytes * 2
            # if len(data) != expected_hex_len:
            #    print(f"Warning: Pushed data length ({len(data)}) doesn't match expected hex length ({expected_hex_len}) for {op}")
            pc += 1 # Increment pc again for the data
            stack.append(data)
            operation_description = f"Executing: {op} {data}" # More specific description

        elif op == "OP_CHECKSEQUENCEVERIFY":
            # Needs tx.nSequence and stack top value.
            # Simulates SUCCESS by consuming the stack value.
            # Assumes the actual nSequence check would pass.
            if len(stack) < 1: raise IndexError("Stack underflow for OP_CHECKSEQUENCEVERIFY")
            print(f"  (Simulating OP_CHECKSEQUENCEVERIFY: Consuming stack top '{stack[-1]}', assuming check passes)")
            stack.pop() # Pop the sequence value required by CSV

        elif op == "OP_DROP":
             if len(stack) < 1: raise IndexError("Stack underflow for OP_DROP")
             stack.pop()

        elif op == "OP_TOALTSTACK":
            if not stack: raise IndexError("Stack underflow for OP_TOALTSTACK")
            alt_stack.append(stack.pop())

        elif op == "OP_FROMALTSTACK":
            if not alt_stack: raise IndexError("AltStack underflow for OP_FROMALTSTACK")
            stack.append(alt_stack.pop())

        # OP_2DUP is not in the new script, can be removed if desired
        # elif op == "OP_2DUP":
        #     if len(stack) < 2: raise IndexError("Stack underflow for OP_2DUP")
        #     b = stack[-1]
        #     a = stack[-2]
        #     stack.append(a)
        #     stack.append(b)

        elif op == "OP_CAT":
            # Note: OP_CAT is disabled in non-Tapscript standard Bitcoin script
            if len(stack) < 2: raise IndexError("Stack underflow for OP_CAT")
            a = stack.pop() # Top item
            b = stack.pop() # Second item
            # Ensure both are strings before concatenating
            if not isinstance(a, str) or not isinstance(b, str):
                 raise TypeError(f"OP_CAT requires string inputs, got {type(b)} and {type(a)}")
            stack.append(b + a) # Correct order: second popped + first popped

        elif op == "OP_SWAP":
            if len(stack) < 2: raise IndexError("Stack underflow for OP_SWAP")
            a = stack.pop()
            b = stack.pop()
            stack.append(a)
            stack.append(b)

        elif op == "OP_SHA256":
            if not stack: raise IndexError("Stack underflow for OP_SHA256")
            item_to_hash = stack.pop()
            h = sha256_hex(item_to_hash)
            stack.append(h)

        elif op == "OP_DUP":
            if not stack: raise IndexError("Stack underflow for OP_DUP")
            stack.append(stack[-1])

        elif op == "OP_ROT":
            # Rotates the top three items (a b c -> b c a)
            if len(stack) < 3: raise IndexError("Stack underflow for OP_ROT")
            c = stack.pop() # Top
            b = stack.pop() # Second
            a = stack.pop() # Third
            stack.append(b)
            stack.append(c)
            stack.append(a) # Third becomes top

        elif op == "OP_EQUALVERIFY":
            if len(stack) < 2: raise IndexError("Stack underflow for OP_EQUALVERIFY")
            a = stack.pop()
            b = stack.pop()
            if a == b:
                print(f"  (OP_EQUALVERIFY: '{a}' == '{b}' -> PASS)")
                # Verification passes, do nothing to the stack
            else:
                print(f"  (OP_EQUALVERIFY: '{a}' != '{b}' -> FAIL)")
                raise ValueError("OP_EQUALVERIFY failed") # Stop execution

        elif op == "OP_CHECKSIG":
            # Simulates success: Pop public key and signature
            # Technically pushes True ('01' or b'\x01'), but for this trace
            # we just consume inputs as the script often ends here.
            if len(stack) < 2: raise IndexError("Stack underflow for OP_CHECKSIG (needs pubkey and signature)")
            sig = stack.pop()
            pubkey = stack.pop()
            print(f"  (Simulating OP_CHECKSIG: Consuming signature '{sig[:10]}...' and pubkey '{pubkey[:10]}...', assuming VALID)")
            # To be more accurate, push '01' (representing True) if script continues
            # stack.append('01')
            # However, since this is the LAST opcode, leaving the stack empty is fine.

        else:
            print(f"Unknown or Unimplemented OP Code: {op}")
            # Choose how to handle: stop or skip
            # exit() # Option 1: Stop execution
            raise NotImplementedError(f"Opcode {op} not implemented in this simulator.") # Option 2: Raise error


        # --- Debug Output ---
        # Use the more specific description if available
        debug_stack(operation_description)

    except (IndexError, ValueError, TypeError, NotImplementedError) as e:
        print(f"\n--- SCRIPT FAILED at '{operation_description}' ---")
        print(f"Error: {e}")
        print("Stacks *before* the failing operation:")
        # To show state *before* failure, ideally print *before* executing,
        # but printing after within the except block shows the state *after*
        # potential pops but *before* the final push/modification causing the error.
        # For clarity, let's just show the final stacks *at the point of failure*.
        print(f"Stack ({len(stack)} items, top first): {list(reversed(stack))}")
        print(f"AltStack ({len(alt_stack)} items, top first): {list(reversed(alt_stack))}")
        print("-" * 30)
        exit() # Stop execution on failure

# --- Final State ---
print("\n--- Script Execution Finished ---")
debug_stack("Final State")

# Optional: Check final stack state
# A standard successful script execution often leaves a single TRUE value ('01') on the stack.
# However, scripts ending in VERIFY opcodes might leave an empty stack.
# For CHECKSIG (non-VERIFY), it should leave '01' unless it's the absolute last op.
if len(stack) == 0:
     print("\nScript finished successfully with an empty stack (typical for scripts ending in VERIFY or a final CHECKSIG).")
# elif len(stack) == 1 and stack[0] == '01':
#     print("\nScript finished successfully with TRUE ('01') on stack.")
else:
     print("\nScript finished, but final stack state is not empty.")