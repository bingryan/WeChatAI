import { readFileSync, writeFileSync } from 'fs';

const targetVersion = process.env.VERSION;

// package.json
let packageInfo = JSON.parse(readFileSync('package.json', 'utf8'));
packageInfo.version = targetVersion;
writeFileSync('package.json', JSON.stringify(packageInfo, null, '\t'));

// tauri.conf.json
let tauriConf = JSON.parse(readFileSync('src-tauri/tauri.conf.json', 'utf8'));
tauriConf.package.version = targetVersion;
writeFileSync(
	'src-tauri/tauri.conf.json',
	JSON.stringify(tauriConf, null, '\t')
);

// Cargo.toml
let cargoFile = readFileSync('src-tauri/Cargo.toml').toString().split('\n');
cargoFile[2] = `version = "${targetVersion}"`;
writeFileSync('src-tauri/Cargo.toml', cargoFile.join('\n'));
