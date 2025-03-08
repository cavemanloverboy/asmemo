

# [num_accounts u64][[account info (88 bytes)]][account data][rent_epoch] .. [next account] .. [ix data len] [ix data] [program_id]

.globl entrypoint
entrypoint:

    # Initial
    # r0 = 0
    # r1 = ptr to num accounts (u64)
    # r2-r9 = 0

    # After operation
    # r0 = 0
    # r1 = ptr to num accounts (u64)
    # r2 = num_accounts (u64)
    # r3-r9 = 0
    # Load num accounts
    ldxdw r2, [r1 + 0]
        

    # Now, loop through accounts and check signer byte is set
    account_loop:
        jeq r2, 0, log_message

        # Load duplicate byte into r3 and check for nondup
        ldxb r3, [r1 + 8]
        jne r3, 255, duplicate_account_error
        
        # If nondup, load signer byte into r3 and check its nonzero
        ldxb r3, [r1 + 9]
        jeq r3, 0, nonsigner_error

        # Now load account data length into r3
        # round up to nearest multiple of 8
        # and increment cursor by this much (and account info and rent epoch and padding)
        ldxdw r3, [r1 + 88]
        add64 r3, 7
        and64 r3, 0xFFFFFFFFFFFFFFF8
        add64 r1, 88 + 10240 + 8 // account info, padding, rent epoch
        add64 r1, r3 // data

        sub64 r2, 1 // Decrement account counter
        ja account_loop
    
    log_message:
        # After this operation
        # r0 = 0
        # r1 = ptr to start of ix data
        # r2 = len of ix data
        # r3-r9 = 0
        # Load ix data length (this assumes num_accounts is zero)
        ldxdw r2, [r1 + 8]
        add64 r1, 16 // skip over num_accounts (8), ix data length (8)


    # TODO: validate ascii
    call sol_log_

    exit

    duplicate_account_error:
        lddw r0, 420
        exit

    nonsigner_error:
        lddw r0, 69
        exit

.rodata
    message: .ascii "Hello, Solana!"
