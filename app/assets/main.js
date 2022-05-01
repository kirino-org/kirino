/*
    Using JavaScript is about as enjoyable as stepping in dog crap on a Saturday morning walk.
*/

let i = 0;

let Http = new XMLHttpRequest;

function addFeed(id) {
    Http.open("POST", "/api/libraries/1/add");
    Http.send(JSON.stringify({
        "original_id": id,
        "fetcher": "invidious",
    }));
}