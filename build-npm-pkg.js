const childProcess = require('child_process');
const path = require('path');
const fs = require('fs');

const execute = (command, cwd = undefined) => {
  const ret = childProcess.spawnSync(command, {
    shell: true,
    stdio: 'inherit',
    cwd
  });
  
  if (ret.error) {
    console.error(JSON.stringify(ret.error));
    process.exit(1);
  }
  
  if (ret.status !== 0) process.exit(1);
};

const OUT_DIR = path.resolve(__dirname, 'semio-record-js');

// Create TS schema
execute(`cargo run -- -o ${OUT_DIR}`);

// Inject WASM
execute(`wasm-pack build -d ${OUT_DIR} ${path.join(__dirname, 'crates', 'semio-record')} --features js`);

// Compile TS
execute(`tsc`, OUT_DIR);

// Modify package.json files
const packageJsonPath = path.join(OUT_DIR, 'package.json');
const pkgJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
pkgJson.files = [
  ...pkgJson.files,
  './**/*.ts'
];
fs.writeFileSync(packageJsonPath, JSON.stringify(pkgJson, null, 2));

