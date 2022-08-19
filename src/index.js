import aptos from '@/utils/aptos';
import Contract from "@/utils/contract";

(async () => {
    await aptos.updateClient(process.env.FULL_NODE_URL);

    const contract = new Contract(process.env.CONTRACT_ADDRESS);
    contract.handleEvents();
})()