window.addEventListener('load', () => {
    let canvas = document.getElementById("maincanvas");

    canvas.style.width = window.innerWidth + "px";
    canvas.style.height = window.innerHeight + "px";

    window.addEventListener("resize", (ev) => {
        console.log(ev);
        canvas.style.width = ev.target.innerWidth + "px";
        canvas.style.height = ev.target.innerHeight + "px";
    });

    /*import("./assets/bin/pong_wasm.js").then(e => {
        console.log(e);
        e.default()
    });*/
  })