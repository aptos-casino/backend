import aptos from './utils/aptos.js';
import Contract from "./utils/contract.js";
import dotenv from 'dotenv';

dotenv.config();

(async () => {
    await aptos.updateClient(process.env.FULL_NODE_URL);

    const contract = new Contract();
    contract.handleEvents();
})()