export type SSFlow = {
  "address": "GLYAbBMPsEdsDcTtPrz3nM3ZZzy4H7KCA7YunF7R8hKU",
  "metadata": {
    "name": "ss_transfer_sol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "SS Transfer Sol"
  },
  "instructions": [
    {
      "name": "initialize_to",
      "discriminator": [
        146,
        1,
        146,
        208,
        47,
        237,
        7,
        110
      ],
      "accounts": [
        {
          "name": "to_account",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  95,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "base"
              }
            ]
          }
        },
        {
          "name": "base",
          "writable": true,
          "signer": true
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "to"
        },
        {
          "name": "token_program",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associated_token_program",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "transfer_sol",
      "discriminator": [
        78,
        10,
        236,
        247,
        109,
        117,
        21,
        76
      ],
      "accounts": [
        {
          "name": "from",
          "writable": true,
          "signer": true
        },
        {
          "name": "to_account",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  111,
                  95,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "to_account.base",
                "account": "To"
              }
            ]
          }
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "To",
      "discriminator": [
        151,
        25,
        60,
        213,
        86,
        79,
        129,
        235
      ]
    }
  ],
  "events": [
    {
      "name": "InitializeToEvent",
      "discriminator": [
        255,
        160,
        98,
        129,
        241,
        93,
        24,
        45
      ]
    },
    {
      "name": "TransferSOLEvent",
      "discriminator": [
        152,
        30,
        30,
        242,
        53,
        48,
        184,
        141
      ]
    }
  ],
  "types": [
    {
      "name": "InitializeToEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "to",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "To",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "base",
            "type": "pubkey"
          },
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "to",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          }
        ]
      }
    },
    {
      "name": "TransferSOLEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "to",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ]
}
