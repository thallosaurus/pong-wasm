const { app, BrowserWindow } = require('electron');
const path = require('path');

const createWindow = () => {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.electron.js')
          }
    })

    win.loadFile('./assets/index.html');
}

app.whenReady().then(() => {
    createWindow()
})