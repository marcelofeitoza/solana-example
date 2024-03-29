// OFF CHAIN
import {
    Keypair,
    Connection,
    PublicKey,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction
} from "@solana/web3.js";
import { serialize } from 'borsh';
import fs from "mz/fs";
import path from "path";
import { Buffer } from 'buffer';
import os from 'os';

const PROGRAM_KEYPAIR_PATH = path.join(path.resolve(__dirname, '../../dist/program'), "program-keypair.json");
const TRIGGER_KEYPAIR_PATH = path.resolve(os.homedir(), '.config/solana/id.json');

enum ActionKind {
    StartAuction = 0, // Discriminant for StartAuction
    MakeBid = 1, // Discriminant for MakeBid
    FinalizeAuction = 2, // Discriminant for FinalizeAuction
}

class Action {
    kind: ActionKind;
    amount: number | null;

    constructor(action: string, amount?: number) {
        switch (action) {
            case 'StartAuction':
                this.kind = ActionKind.StartAuction;
                this.amount = null;
                break;
            case 'MakeBid':
                this.kind = ActionKind.MakeBid;
                if (amount === undefined) {
                    throw new Error('Amount is required for MakeBid action');
                }
                this.amount = amount;
                break;
            case 'FinalizeAuction':
                this.kind = ActionKind.FinalizeAuction;
                this.amount = null;
                break;
            default:
                throw new Error(`Unknown action: ${action}`);
        }
    }

    serialize(): Buffer {
        const buffer = Buffer.alloc(1 + (this.amount !== null ? 8 : 0));
        buffer.writeUInt8(this.kind, 0);
        if (this.amount !== null) {
            buffer.writeBigUInt64LE(BigInt(this.amount), 1);
        }
        return buffer;
    }
}

async function main() {
    console.log("Opening client keypair...");

    let connection = new Connection("https://api.devnet.solana.com", 'confirmed');

    const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, { encoding: "utf-8" });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const programKeypair = Keypair.fromSecretKey(secretKey);
    let programId: PublicKey = programKeypair.publicKey;

    const triggerSecretKeyString = await fs.readFile(TRIGGER_KEYPAIR_PATH, { encoding: "utf-8" });
    const triggerSecretKey = Uint8Array.from(JSON.parse(triggerSecretKeyString));
    const triggerKeypair = Keypair.fromSecretKey(triggerSecretKey);

    console.log("Balance of trigger account:", await connection.getBalance(triggerKeypair.publicKey));
    console.log("Calling program: ", programId.toBase58());

    // Transactions
    let actions = [
        new Action('StartAuction'),
        new Action('MakeBid', 100), // Replace 100 with the actual bid amount
        new Action('FinalizeAuction')
    ];

    for (let action of actions) {
        let instructionData = action.serialize();
        let instruction = new TransactionInstruction({
            keys: [{ pubkey: triggerKeypair.publicKey, isSigner: true, isWritable: false }],
            programId,
            data: instructionData
        });

        await sendAndConfirmTransaction(
            connection,
            new Transaction().add(instruction),
            [triggerKeypair]
        );
    }
}

main().catch(err => {
    console.error(err);
});

// async function main() {
//     console.log("Opening client keypair...");

//     let connection = new Connection("https://api.devnet.solana.com", 'confirmed');

//     const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, { encoding: "utf-8" });
//     const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
//     const programKeypair = Keypair.fromSecretKey(secretKey);
//     let programId: PublicKey = programKeypair.publicKey;

//     const triggerSecretKeyString = await fs.readFile(TRIGGER_KEYPAIR_PATH, { encoding: "utf-8" });
//     const triggerSecretKey = Uint8Array.from(JSON.parse(triggerSecretKeyString));
//     const triggerKeypair = Keypair.fromSecretKey(triggerSecretKey);

//     console.log("Balance of trigger account:", await connection.getBalance(triggerKeypair.publicKey));

//     if ((await connection.getBalance(triggerKeypair.publicKey)) === 0) {
//         try {
//             console.log("Trigger Keypair: ", triggerKeypair.publicKey.toBase58());

//             const airdropRequest = await connection.requestAirdrop(
//                 triggerKeypair.publicKey,
//                 LAMPORTS_PER_SOL
//             );

//             const latestBlockHash = await connection.getLatestBlockhash();
//             await connection.confirmTransaction({
//                 blockhash: latestBlockHash.blockhash,
//                 lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//                 signature: airdropRequest
//             });
//         } catch (error) {
//             console.error("Error requesting airdrop:", error);
//         }
//     }

//     console.log("--Pinging program--");

//     let data = Buffer.from("ping");

//     const instruction = new TransactionInstruction({
//         keys: [{ pubkey: triggerKeypair.publicKey, isSigner: true, isWritable: false }],
//         programId,
//         // data: Buffer.from("marcelo")
//     });
//     try {
//         await sendAndConfirmTransaction(
//             connection,
//             new Transaction().add(instruction),
//             [triggerKeypair]
//         );
//     } catch (error) {
//         console.error("Error sending transaction:", error);
//     }
// }

// main().then(
//     () => console.log("Finished execution"),
//     err => {
//         console.error(err);
//     }
// );