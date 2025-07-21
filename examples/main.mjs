import { startRootListener } from '../index.js'

startRootListener((err, value) => {
    if(err){
        console.log("received error, break now!");
        return;
    }
    console.log("Received value from rust: ", value);
});