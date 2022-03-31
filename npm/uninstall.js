const fs = require('fs');

function getBinary() {
  try {
    const getBinary = require("./getBinary");
    return getBinary();
  } catch (err) {}
}

const binary = getBinary();
if (binary) {
  try {
    fs.unlinkSync(binary.binaryPath);
  } catch (err) {
    
  }
}
