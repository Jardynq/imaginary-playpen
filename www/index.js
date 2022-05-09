import("../pkg").then(application => {
    console.log("wasm loaded succesfully");
    let app = application.create();
    app.init();

    window.eval_rs = application.eval_rs;
}).catch(e => console.error("error executing wasm: ", e));
