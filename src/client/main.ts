// OFF CHAIN
import {
    Keypair,
    Connection,
    PublicKey,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction
} from "@solana/web3.js";
import { Schema, serialize } from 'borsh';
import fs from "mz/fs";
import path from "path";
import { Buffer } from 'buffer';
import os from 'os';

const PROGRAM_KEYPAIR_PATH = path.join(path.resolve(__dirname, '../../dist/program'), "program-keypair.json");
const TRIGGER_KEYPAIR_PATH = path.resolve(os.homedir(), '.config/solana/id.json');

enum ActionKind {
    BatteryReport = 0
}

class Action {
    kind: ActionKind;
    amount: number | null;

    static schema = {
        struct: {
            kind: 'u8',
            amount: 'u64'
        }
    }

    constructor(action: string, amount?: number) {
        switch (action) {
            case 'BatteryReport':
                this.kind = ActionKind.BatteryReport;
                if (amount === undefined) {
                    throw new Error('Amount is required for BatteryReport action');
                }
                this.amount = amount;
                break;
            default:
                throw new Error(`Unknown action: ${action}`);
        }
    }

    serialize(): Buffer {
        return Buffer.from(serialize(Action.schema, this));
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
        new Action('BatteryReport', 100)
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
