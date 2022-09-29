const childProcess = require('child_process');
const path = require('path');

const execute = command => {
  const ret = childProcess.spawnSync(command, {
    shell: true,
    stdio: 'inherit',
  });
  
  if (ret.error) {
    console.error(JSON.stringify(ret.error));
    process.exit(1);
  }
  
  if (ret.status !== 0) process.exit(1);
};

const OUT_DIR = path.resolve(__dirname, 'semio-record-js');

execute(`cargo run -- -o ${OUT_DIR}`);
execute(`wasm-pack build -d ${OUT_DIR} ${path.join(__dirname, 'crates', 'semio-record')} --features js`);


