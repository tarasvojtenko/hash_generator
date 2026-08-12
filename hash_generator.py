### 1. `hash_generator.py` (Python)

```python
# hash_generator.py — Python версия

import hashlib
import argparse
import sys
import os
from colorama import init, Fore, Style

init(autoreset=True)

def compute_hash(data, algo='sha512'):
    """Вычисляет хеш для байтовых данных."""
    if algo == 'sha512':
        return hashlib.sha512(data).hexdigest()
    elif algo == 'sha256':
        return hashlib.sha256(data).hexdigest()
    elif algo == 'sha384':
        return hashlib.sha384(data).hexdigest()
    elif algo == 'sha1':
        return hashlib.sha1(data).hexdigest()
    elif algo == 'md5':
        return hashlib.md5(data).hexdigest()
    else:
        raise ValueError(f"Неподдерживаемый алгоритм: {algo}")

def hash_file(filename, algo='sha512', buffer_size=8192):
    """Хеширует файл с прогресс-баром."""
    total_size = os.path.getsize(filename)
    hash_func = getattr(hashlib, algo)
    if not hash_func:
        raise ValueError(f"Неподдерживаемый алгоритм: {algo}")
    hasher = hash_func()
    processed = 0
    with open(filename, 'rb') as f:
        while True:
            chunk = f.read(buffer_size)
            if not chunk:
                break
            hasher.update(chunk)
            processed += len(chunk)
            # Простой прогресс
            if total_size > 1024 * 1024:  # >1MB
                percent = (processed / total_size) * 100
                sys.stderr.write(f"\r⏳ Прогресс: {percent:.1f}%")
                sys.stderr.flush()
    if total_size > 1024 * 1024:
        sys.stderr.write("\n")
    return hasher.hexdigest()

def main():
    parser = argparse.ArgumentParser(description='Hash Generator Pro')
    parser.add_argument('input', nargs='?', help='Строка для хеширования (или путь к файлу с --file)')
    parser.add_argument('--algo', '-a', default='sha512',
                        choices=['sha512', 'sha256', 'sha384', 'sha1', 'md5'],
                        help='Алгоритм хеширования (по умолчанию sha512)')
    parser.add_argument('--format', '-f', default='hex', choices=['hex', 'base64'],
                        help='Формат вывода (hex или base64)')
    parser.add_argument('--file', '-F', help='Файл для хеширования (если указан, игнорирует строку)')
    parser.add_argument('--output', '-o', help='Сохранить хеш в файл')
    parser.add_argument('--compare', '-c', help='Сравнить с эталонным хешем')
    args = parser.parse_args()

    print(f"{Fore.CYAN}🔐 Hash Generator (Python)")
    print(f"Алгоритм: {args.algo.upper()}")

    data = None
    if args.file:
        print(f"📂 Хеширование файла: {args.file}")
        try:
            hex_digest = hash_file(args.file, args.algo)
            data = hex_digest.encode()
        except Exception as e:
            print(f"{Fore.RED}❌ Ошибка: {e}")
            sys.exit(1)
    elif args.input:
        print(f"📝 Входные данные: {args.input}")
        hex_digest = compute_hash(args.input.encode('utf-8'), args.algo)
        data = hex_digest.encode()
    else:
        # Если нет аргументов, читаем stdin
        print("📝 Чтение из STDIN (Ctrl+D для окончания)")
        try:
            content = sys.stdin.buffer.read()
            if not content:
                print(f"{Fore.YELLOW}⚠️ Пустой ввод.")
                sys.exit(1)
            hex_digest = compute_hash(content, args.algo)
            data = hex_digest.encode()
        except KeyboardInterrupt:
            print(f"{Fore.YELLOW}\n⚠️ Прервано.")
            sys.exit(1)

    # Конвертация в base64, если нужно
    if args.format == 'base64':
        import base64
        result = base64.b64encode(data).decode('ascii')
    else:
        result = data.decode('ascii')

    print(f"{Fore.GREEN}Хеш ({args.format}):")
    print(result)

    # Сравнение
    if args.compare:
        if args.format == 'base64':
            expected = args.compare
        else:
            expected = args.compare.lower()
        if result == expected:
            print(f"{Fore.GREEN}✅ Хеши совпадают!")
        else:
            print(f"{Fore.RED}❌ Хеши не совпадают!")

    # Сохранение в файл
    if args.output:
        try:
            with open(args.output, 'w') as f:
                f.write(result + '\n')
            print(f"{Fore.GREEN}💾 Сохранено в {args.output}")
        except Exception as e:
            print(f"{Fore.RED}❌ Ошибка сохранения: {e}")

if __name__ == "__main__":
    main()
