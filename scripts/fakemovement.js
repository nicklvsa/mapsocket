import fetch from 'node-fetch';

const buildFc = (coords) => {
    return {
        type: 'FeatureCollection',
        features: [
            {
                type: 'Feature',
                properties: {
                    desc: 'You',
                },
                geometry: {
                    type: 'Point',
                    coordinates: coords,
                }
            }
        ]
    };
}

const sleep = async (amount = 1000) => {
    return new Promise((res, rej) => {
        setTimeout(res, amount);
    });
};

const main = async () => {
    const begin = [0, 24];

    const runFetch = async (username, coords) => {
        const config = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                sender_user_id: username,
                topic: "map",
                message: JSON.stringify(buildFc(coords)),
            })
        };
    
        await fetch('http://localhost:8080/publish', config);
    };

    let user = 'nicklvsa';

    await runFetch(user, begin);
    await sleep(2000);

    for (let i = 0; i < 1000; i++) {
        await runFetch(user, begin);
        await sleep(60);
        begin[0] += 0.5;
        begin[1] += 0.110;
    }
};

(async () => {
    await main();
})();