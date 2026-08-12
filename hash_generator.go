// hash_generator.go — Go версия

package main

import (
	"crypto/md5"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/base64"
	"encoding/hex"
	"flag"
	"fmt"
	"hash"
	"io"
	"os"
)

func computeHash(data []byte, algo string) string {
	var h hash.Hash
	switch algo {
	case "sha512":
		h = sha512.New()
	case "sha256":
		h = sha256.New()
	case "sha384":
		h = sha512.New384()
	case "sha1":
		h = sha1.New()
	case "md5":
		h = md5.New()
	default:
		fmt.Fprintf(os.Stderr, "Неподдерживаемый алгоритм: %s\n", algo)
		os.Exit(1)
	}
	h.Write(data)
	return hex.EncodeToString(h.Sum(nil))
}

func hashFile(filename, algo string) (string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return "", err
	}
	defer file.Close()
	var h hash.Hash
	switch algo {
	case "sha512":
		h = sha512.New()
	case "sha256":
		h = sha256.New()
	case "sha384":
		h = sha512.New384()
	case "sha1":
		h = sha1.New()
	case "md5":
		h = md5.New()
	default:
		return "", fmt.Errorf("неподдерживаемый алгоритм: %s", algo)
	}
	buf := make([]byte, 8192)
	for {
		n, err := file.Read(buf)
		if err != nil && err != io.EOF {
			return "", err
		}
		if n == 0 {
			break
		}
		h.Write(buf[:n])
	}
	return hex.EncodeToString(h.Sum(nil)), nil
}

func main() {
	algo := flag.String("algo", "sha512", "Алгоритм (sha512, sha256, sha384, sha1, md5)")
	format := flag.String("format", "hex", "Формат вывода (hex, base64)")
	filePath := flag.String("file", "", "Файл для хеширования")
	output := flag.String("output", "", "Сохранить хеш в файл")
	compare := flag.String("compare", "", "Сравнить с эталонным хешем")
	flag.Parse()

	fmt.Println("\x1b[36m🔐 Hash Generator (Go)\x1b[0m")
	fmt.Printf("Алгоритм: %s\n", *algo)

	var hexDigest string
	var err error

	if *filePath != "" {
		fmt.Printf("📂 Хеширование файла: %s\n", *filePath)
		hexDigest, err = hashFile(*filePath, *algo)
		if err != nil {
			fmt.Printf("\x1b[31m❌ Ошибка: %v\x1b[0m\n", err)
			os.Exit(1)
		}
	} else if flag.NArg() > 0 {
		input := flag.Arg(0)
		fmt.Printf("📝 Входные данные: %s\n", input)
		hexDigest = computeHash([]byte(input), *algo)
	} else {
		// Чтение из stdin
		fmt.Println("📝 Чтение из STDIN (Ctrl+D для окончания)")
		stat, _ := os.Stdin.Stat()
		if (stat.Mode() & os.ModeCharDevice) == 0 {
			// данные из пайпа
			data, err := io.ReadAll(os.Stdin)
			if err != nil {
				fmt.Printf("\x1b[31m❌ Ошибка чтения: %v\x1b[0m\n", err)
				os.Exit(1)
			}
			hexDigest = computeHash(data, *algo)
		} else {
			fmt.Println("Введите строку (Enter для завершения):")
			var input string
			fmt.Scanln(&input)
			if input == "" {
				fmt.Println("\x1b[33m⚠️ Пустой ввод.\x1b[0m")
				os.Exit(1)
			}
			hexDigest = computeHash([]byte(input), *algo)
		}
	}

	// Конвертация
	var result string
	if *format == "base64" {
		decoded, _ := hex.DecodeString(hexDigest)
		result = base64.StdEncoding.EncodeToString(decoded)
	} else {
		result = hexDigest
	}

	fmt.Printf("\x1b[32mХеш (%s):\x1b[0m\n", *format)
	fmt.Println(result)

	if *compare != "" {
		if result == *compare {
			fmt.Println("\x1b[32m✅ Хеши совпадают!\x1b[0m")
		} else {
			fmt.Println("\x1b[31m❌ Хеши не совпадают!\x1b[0m")
		}
	}

	if *output != "" {
		err := os.WriteFile(*output, []byte(result+"\n"), 0644)
		if err != nil {
			fmt.Printf("\x1b[31m❌ Ошибка сохранения: %v\x1b[0m\n", err)
		} else {
			fmt.Printf("\x1b[32m💾 Сохранено в %s\x1b[0m\n", *output)
		}
	}
}
