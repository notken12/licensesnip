// getBinary.js
//
// MIT License
//
// Copyright (c) 2022 Ken Zhou
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
