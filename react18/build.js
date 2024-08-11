const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// 检查 wasm-pack 是否已安装
function checkWasmPack() {
  try {
    execSync('wasm-pack --version', { stdio: 'ignore' });
    return true;
  } catch (error) {
    return false;
  }
}

// 安装 wasm-pack
function installWasmPack() {
  console.log('Installing wasm-pack...');
  try {
    execSync('cargo install wasm-pack', { stdio: 'inherit' });
    console.log('wasm-pack installed successfully.');
  } catch (error) {
    console.error('Failed to install wasm-pack:', error.message);
    process.exit(1);
  }
}

const packagesDir = path.join(__dirname, 'packages');
const packages = fs.readdirSync(packagesDir, { withFileTypes: true })
  .filter(dirent => dirent.isDirectory())
  .map(dirent => dirent.name);

const buildCommands = {
  'react': 'wasm-pack build --release --out-name jsx-dev-runtime',
  'react-dom': 'wasm-pack build --release',
  // 为其他包添加自定义命令
  'default': 'cargo build --release'
};

// 主函数
async function main() {
  // 检查并安装 wasm-pack
  if (!checkWasmPack()) {
    installWasmPack();
  }

  // 构建每个包
  for (const pkg of packages) {
    console.log(`Building ${pkg}...`);
    const packageDir = path.join(packagesDir, pkg);
    const command = buildCommands[pkg] || buildCommands.default;
    try {
      execSync(command, { cwd: packageDir, stdio: 'inherit' });
    } catch (error) {
      console.error(`Failed to build ${pkg}: ${error.message}`);
      process.exit(1);
    }
  }
}

// 运行主函数
main().catch(error => {
  console.error('An error occurred:', error);
  process.exit(1);
});