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
    ldxdw r2, [r1 + 0]                                              // 1 CU
    jne r2, 0, fail_nonzero_accounts                                // 2 CU

    # After this operation
    # r0 = 0
    # r1 = ptr to start of ix data
    # r2 = len of ix data
    # r3-r9 = 0
    # Load ix data length (this assumes num_accounts is zero)
    ldxdw r2, [r1 + 8]                                              // 3 CU 
    add64 r1, 16 // skip over num_accounts (8), ix data length (8)  // 4 CU


    # TODO: validate ascii
    call sol_log_                                                   // 105 CU

    exit                                                            // 106 CU


    fail_nonzero_accounts:
        exit                                                        // 3 CU

.rodata
    message: .ascii "Hello, Solana!"
