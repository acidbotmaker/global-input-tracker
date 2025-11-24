const os = require('os')
const platform = os.platform()
const arch = os.arch()

let filename

if (platform === 'darwin' && arch === 'x64') filename = 'darwin-x64.node'
else if (platform === 'darwin' && arch === 'arm64') filename = 'darwin-arm64.node'
else if (platform === 'linux' && arch === 'x64') filename = 'linux-x64.node'
else if (platform === 'linux' && arch === 'arm64') filename = 'linux-arm64.node'
else if (platform === 'win32' && arch === 'x64') filename = 'win32-x64.node'
else throw new Error(`Unsupported platform: ${platform} ${arch}`)

module.exports = require(`./dist/${filename}`)
