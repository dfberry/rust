const axios = require('axios');

// URL of the endpoint to stress test.
const URL = 'http://localhost:3001';

// Number of requests to send concurrently every interval.
const CONCURRENCY = 100;

// Interval between batches in milliseconds.
const INTERVAL_MS = 1000;

async function sendRequest(i) {
    try {
        const response = await axios.get(URL, {
            headers: { 'Content-Type': 'application/json' }
        });
        console.log(`Request ${i} succeeded:`, response.status);
    } catch (error) {
        console.error(`Request ${i} failed:`, error.message);
    }
}

function startStressTest() {
    setInterval(() => {
        console.log(`Sending ${CONCURRENCY} requests...`);
        for (let i = 0; i < CONCURRENCY; i++) {
            sendRequest(i);
        }
    }, INTERVAL_MS);
}

startStressTest();