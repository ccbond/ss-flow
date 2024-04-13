export type SSFlow = {
  "version": "0.1.0",
  "name": "ss_flow",
  "instructions": [
    {
      "name": "initializePool",
      "accounts": [
        {
          "name": "base",
          "isMut": false,
          "isSigner": true,
          "docs": [
            "`base` is used to initialize admin account."
          ]
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintA",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultA",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintB",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultB",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "proportion",
          "type": "u64"
        }
      ]
    },
    {
      "name": "addTokenA",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolTokenAVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenBAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolTokenBVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "refundTokenB",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolTokenAVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenBAta",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolTokenBVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "settle",
      "accounts": [
        {
          "name": "whirlpoolProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "whirlpool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "funder",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "position",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "`position` will be initialized by the `Position` string、pool address and authority nft mint."
          ]
        },
        {
          "name": "authorityNftMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "`authority_nft_mint` is only one withdraw permit of position."
          ]
        },
        {
          "name": "authorityNftAta",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "`authority_nft_token` is the token of authority nft mint."
          ]
        },
        {
          "name": "positionTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolTokenAVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenOwnerAccountB",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenVaultA",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenVaultB",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tickArrayLower",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tickArrayUpper",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAAta",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "precent",
          "type": "u64"
        },
        {
          "name": "liquidityAmount",
          "type": "u128"
        },
        {
          "name": "tokenMaxB",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "Pool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "base",
            "type": "publicKey"
          },
          {
            "name": "admin",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "mintA",
            "type": "publicKey"
          },
          {
            "name": "vaultA",
            "type": "publicKey"
          },
          {
            "name": "mintB",
            "type": "publicKey"
          },
          {
            "name": "vaultB",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "positionCount",
            "type": "u64"
          },
          {
            "name": "proportion",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Position",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "authorityNftMint",
            "type": "publicKey"
          },
          {
            "name": "pool",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "InitializePoolEvent",
      "fields": [
        {
          "name": "admin",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenA",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenB",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "proportion",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "AddTokenAEvent",
      "fields": [
        {
          "name": "payer",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenA",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "RefundTokenBEvent",
      "fields": [
        {
          "name": "payer",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenB",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawTokenAEvent",
      "fields": [
        {
          "name": "payer",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "authorityNftMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "lockedNft",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "remainAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "SettleEvent",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "admin",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenA",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenB",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amountA",
          "type": "u64",
          "index": false
        },
        {
          "name": "amountB",
          "type": "u64",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "Unauthorized",
      "msg": "Unauthorized"
    },
    {
      "code": 6001,
      "name": "InvalidInitAmount",
      "msg": "Invalid interver when initializing pool"
    },
    {
      "code": 6002,
      "name": "EndTimeMustBeGreaterThanStartTime",
      "msg": "End time must be greater than start_time"
    },
    {
      "code": 6003,
      "name": "StartTimeMustBeGreaterThanCurrentTime",
      "msg": "Start time must be greater than current_time"
    },
    {
      "code": 6004,
      "name": "NftMintAlreadyInitialized",
      "msg": "Nft mint already initialized"
    },
    {
      "code": 6005,
      "name": "InvalidNftBalance",
      "msg": "Invalid nft balance"
    },
    {
      "code": 6006,
      "name": "WithdrawAlreadyDone",
      "msg": "Withdraw already done"
    },
    {
      "code": 6007,
      "name": "InvalidWithdrawTime",
      "msg": "Invalid withdraw time"
    },
    {
      "code": 6008,
      "name": "WithdrawPaused",
      "msg": "Withdraw is paused"
    },
    {
      "code": 6009,
      "name": "VerifiedAdminFailed",
      "msg": "Verified admin failed"
    },
    {
      "code": 6010,
      "name": "UnlockPeriodMustBeMultipleOfFreedInterval",
      "msg": "Unlock period must be multiple of freed interval"
    },
    {
      "code": 6011,
      "name": "WithdrawNotStart",
      "msg": "Withdraw not start"
    },
    {
      "code": 6012,
      "name": "PoolWithdrawAlreadyPause",
      "msg": "Pool withdraw already pause"
    },
    {
      "code": 6013,
      "name": "PoolWithdrawAlreadyStart",
      "msg": "Pool withdraw already start"
    },
    {
      "code": 6014,
      "name": "NoAvailableWithdrawToken",
      "msg": "No available withdraw token"
    },
    {
      "code": 6015,
      "name": "InvaildAuthorityNftATA",
      "msg": "Invaild authority nft ata"
    }
  ]
}