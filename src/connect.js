/*
const { invoke } = window.__TAURI__.tauri;

class Handler {
    constructor(){
        //this.invoke = window.__TAURI__.tauri;
    }
}

console.log("Bip")

class IncludedPage {
    constructor(src, state, name, element){
        this.src = src;
        this.state = state;
        this.name = name;
        this.element = element;
    }

    async load(){
        console.log(this.outerHTML)
        this.element.outerHTML = (await (await fetch("test.html")).text()).split("<body>")[1].split("</body>")[0];
    }
}

let includes = [];

for (let includeTag of document.getElementsByTagName("include")){
    console.log(includeTag)
    let attr = includeTag.attributes
    let src = attr["src"].value;
    let state = attr["state"] ? attr["state"].value : "show";
    let name = attr["name"] ? attr["name"].value : src;

    includes.push(new IncludedPage(src, state, name, includeTag));
}

console.log(includes)
Promise.all(includes.map(include => include.load()))


fetch("test.html").then(response => response.text()).then(text => {
    console.log(text.split("<body>")[1].split("</body>")[0]);
});

*/

const { invoke } = window.__TAURI__.tauri;

async function connect(username, password){
    return await invoke("connect", {username: username, password: password});
}


let username = document.querySelector("#username");
let password = document.querySelector("#password");
document.querySelector("#login").addEventListener("click", async (e) => {
    console.log("Nya");
    e.preventDefault();
    try {
        await connect(username.value, password.value);
        console.log("Connected");
        console.log(await invoke("get_informations"));
    } catch (error) {
        console.log(error);
    }
});