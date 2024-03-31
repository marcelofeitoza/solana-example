import {Schema, serialize} from 'borsh';

export enum ActionKind {
    BatteryReport,
    PlaceBid,
    StartAuction,
    FinalizeAuction,
}

export class Action {
    kind: ActionKind;
    id: string | null |undefined;
    latitude: number | null |undefined;
    longitude: number | null |undefined;
    max_capacity: number | null |undefined;
    battery_level: number | null |undefined;
    amount: number | null | undefined;

    static batteryReportSchema: Schema = {
        struct: {
            kind: 'u8',
            id: 'string',
            latitude: 'f32',
            longitude: 'f32',
            max_capacity: 'f32',
            battery_level: 'f32',
        }
    }

    static makeBidSchema: Schema = {
        struct: {
            kind: 'u8',
            amount: 'u64'
        }
    }

    constructor(kind: ActionKind, options: ActionOptions = {}) {
        this.kind = kind;
        this.id = options.id;
        this.latitude = options.latitude;
        this.longitude = options.longitude;
        this.max_capacity = options.max_capacity;
        this.battery_level = options.battery_level;
        this.amount = options.amount;

        switch (this.kind) {
            case ActionKind.BatteryReport:
                if (
                    this.id === undefined || this.latitude === undefined || this.longitude === undefined || this.max_capacity === undefined || this.battery_level === undefined
                ) {
                    throw new Error("Missing required fields for BatteryReport");
                }
                break;

            case ActionKind.StartAuction:
                this.kind = ActionKind.StartAuction;
                this.amount = null;
                break;
            case ActionKind.PlaceBid:
                this.kind = ActionKind.PlaceBid;
                if (this.amount === undefined) {
                    throw new Error('Amount is required for MakeBid action');
                }
                break;
            case ActionKind.FinalizeAuction:
                this.kind = ActionKind.FinalizeAuction;
                this.amount = null;
                break;

            default:
                throw new Error(`Unknown action: ${kind}`);
        }
    }

    serialize(): Buffer {
        switch (this.kind) {
            case ActionKind.BatteryReport:
                return Buffer.from(serialize(Action.batteryReportSchema, {
                    kind: this.kind,
                    id: this.id,
                    latitude: this.latitude,
                    longitude: this.longitude,
                    max_capacity: this.max_capacity,
                    battery_level: this.battery_level
                }));
            case ActionKind.PlaceBid:
                return Buffer.from(serialize(Action.makeBidSchema, {
                    kind: this.kind,
                    amount: this.amount
                }));
            case ActionKind.StartAuction:
            case ActionKind.FinalizeAuction:
                return Buffer.from(serialize({ struct: { kind: 'u8' } }, { kind: this.kind }));
            default:
                throw new Error(`Unknown action: ${this.kind}`);
        }
    }
}

export interface ActionOptions {
    id?: string;
    latitude?: number;
    longitude?: number;
    max_capacity?: number;
    battery_level?: number;
    amount?: number;
}