import http from "k6/http";

//k6 run k6.js --vus 100 --duration 10s
//k6 run k6.js --vus 300 --duration 10s

export default function() {
    const url = `http://localhost:8080`;
    http.get(url);
}