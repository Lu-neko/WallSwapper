(function(){

	class IncludedPage {
		constructor(src, state, loader){
			this.src = src;
			this.state = state;
			this.loader = loader;
			this.elements = null;
		}

		async load(handler){
			const response = await fetch("/"+this.src);
			const text = await response.text();
			const html = text.split("<body>")[1].split("</body>")[0];
			const template = document.createElement('template');
			template.innerHTML = html;

			await handler.load(template);

			this.elements = template.content.children;

			if (this.state == "show"){
				this.loader.replaceChildren(...this.elements);
				import("/"+this.src.split(".")[0]+".js");
			}
		}
	}

	class PageHandler {

		#tauri = null;

		constructor(tauri){
			this.#tauri = tauri;
			this.included = {}
		}

		async load(html){

			let include = {}

			for (let includeTag of html.getElementsByTagName("include")){
				let attr = includeTag.attributes;

				const regex = new RegExp("^[a-zA-Z0-9-_]+\\.html$") 

				if (!attr["src"] || !regex.exec(attr["src"].value)){
					continue;
				}

				let src = attr["src"].value;
				let state = attr["state"] ? attr["state"].value : "show";
				let name = attr["name"] ? attr["name"].value : src;

				if (name in this.included){
					continue;
				}

				include[name] = new IncludedPage(src, state, includeTag);
				this.included[name] = include[name];
			}

			await Promise.all(Object.values(include).map(elem => elem.load(this)));
		}

		async connect(username, password){
			return await this.#tauri("connect", {username: username, password: password});
		}

		async getInformations(){
			const result = await this.#tauri("get_informations", {});

			if (result) {
				return null;
			}
			return null;
		}
	}

	const { invoke } = window.__TAURI__.tauri;

	const handler = new PageHandler(invoke);

	handler.load(document);

})();
