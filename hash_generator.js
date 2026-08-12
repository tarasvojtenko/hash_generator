// hash_generator.js — JavaScript версия

const crypto = require('crypto');
const fs = require('fs');
const readline = require('readline');

function computeHash(data, algo = 'sha512') {
    const hash = crypto.createHash(algo);
    hash.update(data);
    return hash.digest('hex');
}

function hashFile(filename, algo = 'sha512') {
    return new Promise((resolve, reject) => {
        const hash = crypto.createHash(algo);
        const stream = fs.createReadStream(filename);
        let processed = 0;
        const total = fs.statSync(filename).size;
        stream.on('data', (chunk) => {
            hash.update(chunk);
            processed += chunk.length;
            if (total > 1024 * 1024) {
                const percent = (processed / total) * 100;
                process.stderr.write(`\r⏳ Прогресс: ${percent.toFixed(1)}%`);
            }
        });
        stream.on('end', () => {
            if (total > 1024 * 1024) process.stderr.write('\n');
            resolve(hash.digest('hex'));
        });
        stream.on('error', reject);
    });
}

async function main() {
    const args = process.argv.slice(2);
    let algo = 'sha512';
    let format = 'hex';
    let filePath = null;
    let output = null;
    let compare = null;
    let input = null;

    for (let i = 0; i < args.length; i++) {
        const arg = args[i];
        if (arg === '--algo' || arg === '-a') {
            algo = args[++i];
        } else if (arg === '--format' || arg === '-f') {
            format = args[++i];
        } else if (arg === '--file' || arg === '-F') {
            filePath = args[++i];
        } else if (arg === '--output' || arg === '-o') {
            output = args[++i];
        } else if (arg === '--compare' || arg === '-c') {
            compare = args[++i];
        } else if (!arg.startsWith('-')) {
            input = arg;
        }
    }

    console.log('\x1b[36m🔐 Hash Generator (JavaScript)\x1b[0m');
    console.log(`Алгоритм: ${algo.toUpperCase()}`);

    let hexDigest = '';
    if (filePath) {
        console.log(`📂 Хеширование файла: ${filePath}`);
        try {
            hexDigest = await hashFile(filePath, algo);
        } catch (err) {
            console.error(`\x1b[31m❌ Ошибка: ${err.message}\x1b[0m`);
            process.exit(1);
        }
    } else if (input) {
        console.log(`📝 Входные данные: ${input}`);
        hexDigest = computeHash(input, algo);
    } else {
        // stdin
        console.log('📝 Чтение из STDIN (Ctrl+D для окончания)');
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
            terminal: false
        });
        let chunks = [];
        for await (const line of rl) {
            chunks.push(line);
        }
        const data = chunks.join('\n');
        if (!data) {
            console.log('\x1b[33m⚠️ Пустой ввод.\x1b[0m');
            process.exit(1);
        }
        hexDigest = computeHash(data, algo);
    }

    // Конвертация
    let result;
    if (format === 'base64') {
        const buffer = Buffer.from(hexDigest, 'hex');
        result = buffer.toString('base64');
    } else {
        result = hexDigest;
    }

    console.log(`\x1b[32mХеш (${format}):\x1b[0m`);
    console.log(result);

    if (compare) {
        if (result === compare) {
            console.log('\x1b[32m✅ Хеши совпадают!\x1b[0m');
        } else {
            console.log('\x1b[31m❌ Хеши не совпадают!\x1b[0m');
        }
    }

    if (output) {
        try {
            fs.writeFileSync(output, result + '\n');
            console.log(`\x1b[32m💾 Сохранено в ${output}\x1b[0m`);
        } catch (err) {
            console.error(`\x1b[31m❌ Ошибка сохранения: ${err.message}\x1b[0m`);
        }
    }
}

main().catch(console.error);
