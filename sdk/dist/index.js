"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.convertSecretKeyToKeypair = void 0;
const web3_js_1 = require("@solana/web3.js");
const devolt_1 = require("./types/devolt");
const anchor_1 = require("@coral-xyz/anchor");
const program_1 = require("./constants/program");
const helpers_1 = require("./utils/helpers");
const web3_js_2 = require("@solana/web3.js");
const bs58_1 = __importDefault(require("bs58"));
class DevoltClient {
    constructor(connection, wallet) {
        this.connection = connection;
        this.wallet = wallet;
        this.provider = new anchor_1.AnchorProvider(this.connection, this.wallet, anchor_1.AnchorProvider.defaultOptions());
        this.program = new anchor_1.Program(devolt_1.IDL, program_1.DEVOLT_PROGRAM_ID, this.provider);
    }
    batteryReport({ id, latitude, longitude, maxCapacity, batteryLevel }) {
        return __awaiter(this, void 0, void 0, function* () {
            const encodedId = '3';
            const StationPDA = (0, helpers_1.getStationAddressSync)(this.program.programId, encodedId);
            const tx = yield this.program.methods
                .batteryReport({
                id,
                latitude,
                longitude,
                maxCapacity,
                batteryLevel
            })
                .accounts({ signer: this.wallet.publicKey, station: StationPDA })
                .transaction();
            console.log(tx);
            yield this.provider.sendAndConfirm(tx);
        });
    }
    getStation() {
        return __awaiter(this, void 0, void 0, function* () {
            const encodedId = '3';
            const StationPDA = (0, helpers_1.getStationAddressSync)(this.program.programId, encodedId);
            const d = yield this.program.account.station.fetch(StationPDA);
            console.log(d);
        });
    }
}
exports.default = DevoltClient;
const convertSecretKeyToKeypair = (key) => {
    const secretKey = bs58_1.default.decode(key);
    return web3_js_2.Keypair.fromSecretKey(secretKey);
};
exports.convertSecretKeyToKeypair = convertSecretKeyToKeypair;
const secretArray = [
    59, 82, 131, 25, 97, 163, 16, 56, 89, 160, 64, 28, 241, 28, 188, 186, 21, 131,
    230, 113, 252, 100, 208, 60, 137, 240, 43, 85, 254, 217, 68, 149, 163, 5, 193,
    216, 243, 33, 223, 130, 145, 9, 117, 106, 254, 86, 171, 115, 255, 3, 202, 13,
    71, 103, 142, 162, 238, 169, 164, 211, 45, 242, 230, 132
];
const secret = new Uint8Array(secretArray);
const secretHex = Array.from(secret)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');
const secretBase58 = bs58_1.default.encode(Buffer.from(secretHex, 'hex'));
const wallet = new anchor_1.Wallet((0, exports.convertSecretKeyToKeypair)(secretBase58));
console.log(wallet.publicKey.toBase58());
const a = new DevoltClient(new web3_js_1.Connection('https://api.devnet.solana.com'), wallet);
console.log("Caller: ", a.wallet.publicKey.toBase58());
a.batteryReport({
    id: '3',
    latitude: 0,
    longitude: 0,
    maxCapacity: 100,
    batteryLevel: 50
});
a.getStation();
a.batteryReport({
    id: '3',
    latitude: 10,
    longitude: 90,
    maxCapacity: 144363,
    batteryLevel: 99
});
a.getStation();
// import * as anchor from '@coral-xyz/anchor'
// console.log(Buffer.from(anchor.utils.bytes.utf8.encode('station')).toString('hex'))
