import type {
    BroadcastOptions,
    Provider,
    TransactionEnvelope,
    TransactionReceipt,
} from "@saberhq/solana-contrib";

export function loadProvider(): Provider {
    const home: string = process.env.HOME;
    const configFile = fs.readFileSync(
        `${home}/.config/solana/cli/config.yml`,
        "utf8"
    );
    const config = parse(configFile);
    const url = getURL(config.json_rpc_url);
    const wallet = new SignerWallet(keypairFromFile(config.keypair_path));
    const provider = SolanaProvider.init({
        connection: new Connection(url, {
            commitment: "recent",
            disableRetryOnRateLimit: true,
            confirmTransactionInitialTimeout: 60 * 1000,
        }),
        wallet,
        opts: {
            preflightCommitment: "recent",
            commitment: "recent",
        },
    });
    return provider;
}