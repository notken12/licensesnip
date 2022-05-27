const { Binary } = require("binary-install");

const os = require("os");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  // Doesn't support win32 yet. TODO: add build task for it.
  // if (type === 'Windows_NT') return 'win32';
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "macos";

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getBinary() {
  const version = require("../package.json").version;
  const platform = getPlatform();
  const url = `https://github.com/notken12/licensesnip/releases/download/v${version}/licensesnip-${platform}.tar.gz`;
  const name = "licensesnip";
  return new Binary(name, url);
}

module.exports = getBinary;
