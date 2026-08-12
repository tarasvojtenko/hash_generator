<?php
// hash_generator.php — PHP версия

$algo = 'sha512';
$format = 'hex';
$filePath = null;
$output = null;
$compare = null;
$input = null;

$args = array_slice($argv, 1);
for ($i = 0; $i < count($args); $i++) {
    switch ($args[$i]) {
        case '--algo':
        case '-a':
            $algo = $args[++$i];
            break;
        case '--format':
        case '-f':
            $format = $args[++$i];
            break;
        case '--file':
        case '-F':
            $filePath = $args[++$i];
            break;
        case '--output':
        case '-o':
            $output = $args[++$i];
            break;
        case '--compare':
        case '-c':
            $compare = $args[++$i];
            break;
        default:
            if (!str_starts_with($args[$i], '-')) {
                $input = $args[$i];
            }
    }
}

echo "\033[36m🔐 Hash Generator (PHP)\033[0m\n";
echo "Алгоритм: " . strtoupper($algo) . "\n";

function computeHash($data, $algo) {
    return hash($algo, $data);
}

function hashFile($filename, $algo) {
    $total = filesize($filename);
    $processed = 0;
    $ctx = hash_init($algo);
    $fp = fopen($filename, 'rb');
    if (!$fp) {
        throw new Exception("Не удалось открыть файл");
    }
    while (!feof($fp)) {
        $chunk = fread($fp, 8192);
        hash_update($ctx, $chunk);
        $processed += strlen($chunk);
        if ($total > 1024 * 1024) {
            $percent = ($processed / $total) * 100;
            fwrite(STDERR, "\r⏳ Прогресс: " . number_format($percent, 1) . "%");
        }
    }
    fclose($fp);
    if ($total > 1024 * 1024) fwrite(STDERR, "\n");
    return hash_final($ctx);
}

try {
    $hexDigest = '';
    if ($filePath) {
        echo "📂 Хеширование файла: $filePath\n";
        $hexDigest = hashFile($filePath, $algo);
    } elseif ($input) {
        echo "📝 Входные данные: $input\n";
        $hexDigest = computeHash($input, $algo);
    } else {
        echo "📝 Чтение из STDIN (Ctrl+D для окончания)\n";
        $data = file_get_contents('php://stdin');
        if ($data === false || $data === '') {
            echo "\033[33m⚠️ Пустой ввод.\033[0m\n";
            exit(1);
        }
        $hexDigest = computeHash($data, $algo);
    }

    $result = ($format === 'base64') ? base64_encode(hex2bin($hexDigest)) : $hexDigest;

    echo "\033[32mХеш ($format):\033[0m\n";
    echo $result . "\n";

    if ($compare) {
        if ($result === $compare) {
            echo "\033[32m✅ Хеши совпадают!\033[0m\n";
        } else {
            echo "\033[31m❌ Хеши не совпадают!\033[0m\n";
        }
    }

    if ($output) {
        file_put_contents($output, $result . "\n");
        echo "\033[32m💾 Сохранено в $output\033[0m\n";
    }
} catch (Exception $e) {
    echo "\033[31m❌ Ошибка: " . $e->getMessage() . "\033[0m\n";
    exit(1);
}
?>
