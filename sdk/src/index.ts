import { Connection } from '@solana/web3.js'
import { Devolt, IDL } from './types/devolt'
import { AnchorProvider, Program, Wallet, web3 } from '@coral-xyz/anchor'
import { DEVOLT_PROGRAM_ID } from './constants/program'
import { encodeName } from './utils/name'
import { getStationAddressSync } from './utils/helpers'
import { Keypair } from '@solana/web3.js'
import bs58 from 'bs58'

export default class DevoltClient {
    connection: Connection
    wallet: Wallet
    provider: AnchorProvider
    program: Program<Devolt>

    constructor(connection: Connection, wallet: Wallet) {
        this.connection = connection
        this.wallet = wallet
        this.provider = new AnchorProvider(
            this.connection,
            this.wallet,
            AnchorProvider.defaultOptions()
        )
        this.program = new Program<Devolt>(IDL, DEVOLT_PROGRAM_ID, this.provider)
    }

    async batteryReport({
        id,
        latitude,
        longitude,
        maxCapacity,
        batteryLevel
    }: {
        id: string
        latitude: number
        longitude: number
        maxCapacity: number
        batteryLevel: number

    }) {
        const encodedId = '3'
        const StationPDA = getStationAddressSync(this.program.programId, encodedId)
        console.log("Station PDA: ", StationPDA)

        const tx = await this.program.methods
            .batteryReport({
                id,
                latitude,
                longitude,
                maxCapacity,
                batteryLevel
            })
            .accounts({ signer: this.wallet.publicKey, station: StationPDA })
            .transaction()

        console.log("Transaction data: ", tx)
    }


    async getStation() {
        const encodedId = '3'
        const StationPDA = getStationAddressSync(this.program.programId, encodedId)

        const d = await this.program.account.station.fetch(StationPDA)

        console.log(d)
    }
}

export const convertSecretKeyToKeypair = (key: string) => {
    const secretKey = bs58.decode(key)

    return Keypair.fromSecretKey(secretKey)
}

const secretArray: number[] = [
    59, 82, 131, 25, 97, 163, 16, 56, 89, 160, 64, 28, 241, 28, 188, 186, 21, 131,
    230, 113, 252, 100, 208, 60, 137, 240, 43, 85, 254, 217, 68, 149, 163, 5, 193,
    216, 243, 33, 223, 130, 145, 9, 117, 106, 254, 86, 171, 115, 255, 3, 202, 13,
    71, 103, 142, 162, 238, 169, 164, 211, 45, 242, 230, 132]
const secret: Uint8Array = new Uint8Array(secretArray)
const secretHex: string = Array.from(secret)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('')
const secretBase58: string = bs58.encode(Buffer.from(secretHex, 'hex'))
const wallet = new Wallet(convertSecretKeyToKeypair(secretBase58))

console.log(wallet.publicKey.toBase58())

const a = new DevoltClient(
    new Connection('https://api.devnet.solana.com'),
    wallet
)

console.log("Caller: ", a.wallet.publicKey.toBase58())

a.batteryReport({
    id: '3',
    latitude: 0,
    longitude: 0,
    maxCapacity: 100,
    batteryLevel: 50
})
a.getStation()
a.batteryReport({
    id: '3',
    latitude: 10,
    longitude: 90,
    maxCapacity: 144363,
    batteryLevel: 99
})
a.getStation()
// import * as anchor from '@coral-xyz/anchor'
// console.log(Buffer.from(anchor.utils.bytes.utf8.encode('station')).toString('hex'))
