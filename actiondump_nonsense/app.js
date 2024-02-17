function findStrings(data, key) {
    let strings = [];

    if (typeof data === 'string') {
        strings.push(key, data);
    } else if (Array.isArray(data)) {
        data.forEach(item => strings.push(...findStrings(item)));
    } else if (typeof data === 'object') {
        for (const key in data) {
            strings.push(...findStrings(data[key], key));
        }
    }

    return strings;
}

let res = await fetch('http://127.0.0.1:8080/dbc.json')
    .then(response => {
        console.log(response.headers.get("Content-Length"))
       return response.json()
    })
    .then(data => {
        const allStrings = findStrings(data);
        return allStrings;
    })
    .catch(error => console.error(error));

console.log(new Set(res), res);
