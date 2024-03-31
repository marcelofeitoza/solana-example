// OFF CHAIN
import {
    Keypair,
    Connection,
    PublicKey,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction
} from "@solana/web3.js";
import * as fs from "mz/fs";
import * as path from "path";
import * as os from 'os';

import {Action, ActionKind} from './Action';

const PROGRAM_KEYPAIR_PATH = path.join(path.resolve(__dirname, '../../dist/program'), "program-keypair.json");
const TRIGGER_KEYPAIR_PATH = path.resolve(os.homedir(), '.config/solana/id.json');

const setup = async (): Promise<{
    connection: Connection;
    programId: PublicKey;
    triggerKeypair: Keypair;
}> => {
    console.log("Opening client keypair...");

    let connection: Connection = new Connection("https://api.devnet.solana.com", 'confirmed');

    const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, {encoding: "utf-8"});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const programKeypair = Keypair.fromSecretKey(secretKey);

    let programId: PublicKey = programKeypair.publicKey;

    const triggerSecretKeyString = await fs.readFile(TRIGGER_KEYPAIR_PATH, {encoding: "utf-8"});
    const triggerSecretKey = Uint8Array.from(JSON.parse(triggerSecretKeyString));

    const triggerKeypair: Keypair = Keypair.fromSecretKey(triggerSecretKey);

    console.log("Balance of trigger account:", await connection.getBalance(triggerKeypair.publicKey));
    console.log("Calling program: ", programId.toBase58());

    return {connection, programId, triggerKeypair};
}

async function main() {
    const {
        connection,
        programId,
        triggerKeypair
    } = await setup();

    console.log("Trigger account:", triggerKeypair.publicKey.toBase58());

    // let actions = [
    //     new Action(ActionKind.BatteryReport, {
    //         id: "id",
    //         latitude: 1.0,
    //         longitude: 2.01,
    //         max_capacity: 100.0,
    //         battery_level: 99.0
    //     }),
    //     new Action(ActionKind.PlaceBid, {amount: 100}),
    // ];
    // for (let action of actions) {
    //     console.log("\n\n---Calling action: ", ActionKind[action.kind]);
    //
    //     let instructionData = action.serialize();
    //     let instruction = new TransactionInstruction({
    //         keys: [{pubkey: triggerKeypair.publicKey, isSigner: true, isWritable: false}],
    //         programId,
    //         data: instructionData
    //     });
    //
    //     await sendAndConfirmTransaction(
    //         connection,
    //         new Transaction().add(instruction),
    //         [triggerKeypair]
    //     );
    //
    //     console.log("Success---");
    // }

     setInterval(async () => {
        let action = new Action(ActionKind.BatteryReport, {
            id: "id",
            latitude: Math.random() * 180.0 - 90.0,
            longitude: Math.random() * 360.0 - 180.0,
            max_capacity: 100.0,
            battery_level: Math.random() * 100.0
        });

        console.log("\nCalling action: ", ActionKind[action.kind]);

        let instructionData = action.serialize();
        let instruction = new TransactionInstruction({
            keys: [{pubkey: triggerKeypair.publicKey, isSigner: true, isWritable: false}],
            programId,
            data: instructionData
        });

        await sendAndConfirmTransaction(
            connection,
            new Transaction().add(instruction),
            [triggerKeypair]
        );

        console.log("Success");
    }, 1000);
}

main().catch(err => {
    console.error(err);
});
