import aptos from "./aptos.js";
import {AptosAccount, HexString} from "aptos";
import js_sha3 from "js-sha3";
import assert from "assert";
const {sha3_256} = js_sha3;

class Contract {
    constructor() {
        this.onStartGame = this.onStartGame.bind(this)
        this.onInitedBackendSeedHashes = this.onInitedBackendSeedHashes.bind(this)
        this.onInitedClientSeedHashes = this.onInitedClientSeedHashes.bind(this)
        this.onInitedBackendSeed = this.onInitedBackendSeed.bind(this)
        this.onInitedClientSeed = this.onInitedClientSeed.bind(this)
        this.onCompletedGameEvent = this.onCompletedGameEvent.bind(this)

        this.backendPrivateKey = process.env.PRIVATE_KEY;
        this.address = process.env.CONTRACT_ADDRESS;
        assert(!!this.backendPrivateKey && !!this.address, "Wrong env")
        this.backendAccount = new AptosAccount(new HexString(this.backendPrivateKey).toUint8Array(), this.address);
        this.backendSeeds = {};
        this.gameIdToSeedHash = {};
        this.oldGame = {};
    }

    async subscribeOnEvents(sender, eventHandleStruct, field, fromLast, callback) {
        let from = 0;
        if (fromLast) {
            const lastEvent = await aptos.getEvent(this.address, this.address, eventHandleStruct, field, 0, 1)
                .catch(() => {
                    return null;
                });
            if (lastEvent) {
                from = lastEvent["sequence_number"];
            }
        }
        const loop = async () => {
            const lastEvents = await aptos.getEvent(this.address, this.address, eventHandleStruct, field, from, 25)
                .catch(() => {
                    return null;
                });
            if (lastEvents) {
                for (let i = 0; i < lastEvents.length; i++) {
                    const event = lastEvents[i];
                    callback(event);
                    if (from === Number(event.sequence_number)) {
                        from += 1;
                    }
                }
                setTimeout(loop, 1000/*50*/);
            } else {
                setTimeout(loop, 5000);
            }
        };

        setTimeout(loop, 100);
    }

    async onStartGame(eventData) {
        console.log("onStartGame", eventData);

        const gameId = eventData.data["game_id"];

        const setupSeedHash = async () => {
            if (this.oldGame[gameId] === undefined) {
                const {hash} = this.prepareBackendSeed();
                this.gameIdToSeedHash[gameId] = hash;
                await this.SetBackendSeedHash(gameId, hash);
            }
        }
        setTimeout(setupSeedHash, 5000);
    }

    async onInitedBackendSeedHashes(eventData) {
        console.log("onInitedBackendSeedHashes", eventData);
        const gameId = eventData.data["game_id"];
        this.oldGame[gameId] = true;
    }

    async onInitedClientSeedHashes(eventData) {
        console.log("onInitedClientSeedHashes", eventData);
        const gameId = eventData.data["game_id"];
        this.oldGame[gameId] = true;
    }

    async onInitedBackendSeed(eventData) {
        console.log("onInitedBackendSeed", eventData);
        const gameId = eventData.data["game_id"];
        this.oldGame[gameId] = true;
    }

    async onInitedClientSeed(eventData) {
        console.log("onInitedClientSeed", eventData);

        const gameId = eventData.data["game_id"];
        this.oldGame[gameId] = true;
        const hash = this.gameIdToSeedHash[gameId];
        const seed = this.backendSeeds[hash];
        if (!!seed) {
            await this.SetBackendSeed(gameId, seed);
        }
    }

    async onCompletedGameEvent(eventData) {
        console.log("onCompletedGameEvent", eventData);
        const gameId = eventData.data["game_id"];
        this.oldGame[gameId] = true;
    }

    handleEvents() {
        this.subscribeOnEvents(this.address, "Casino::EventsStore",
            "start_game_event", false, this.onStartGame)
            .catch(console.error);
        this.subscribeOnEvents(this.address, "Casino::EventsStore",
            "inited_backend_seed_hashes_event", false, this.onInitedBackendSeedHashes)
            .catch(console.error);
        this.subscribeOnEvents(this.address, "Casino::EventsStore",
            "inited_backend_seed_event", false, this.onInitedBackendSeed)
            .catch(console.error);
        this.subscribeOnEvents(this.address, "Casino::EventsStore",
            "inited_client_seed_event", false, this.onInitedClientSeed)
            .catch(console.error);
        this.subscribeOnEvents(this.address, "Casino::EventsStore",
            "completed_game_event", false, this.onCompletedGameEvent)
            .catch(console.error);
    }

    async SetBackendSeed(gameId, seed) {
        const payload = {
            type: "entry_function_payload",
            function: `${this.address}::Casino::set_backend_seed`,
            type_arguments: [],
            arguments: [BigInt(gameId), seed]
        };
        await aptos.SignAndSubmitTransaction(this.address, this.backendAccount, payload)
            .then(console.log).catch(console.error);
    }

    async SetBackendSeedHash(gameId, hash) {
        const payload = {
            type: "entry_function_payload",
            function: `${this.address}::Casino::set_backend_seed_hash`,
            type_arguments: [],
            arguments: [BigInt(gameId), hash]
        };
        await aptos.SignAndSubmitTransaction(this.address, this.backendAccount, payload)
            .then(console.log).catch(console.error);
    }

    prepareBackendSeed() {
        let seed = new Uint8Array(64);
        for (let i = 0; i < seed.byteLength; i++) {
            seed[i] = Math.ceil(Math.random() * 1000) % 255;
        }
        let hash = sha3_256.array(seed);
        this.backendSeeds[hash] = seed;
        return {
            seed,
            hash
        };
    }
}

export default Contract
