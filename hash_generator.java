// hash_generator.java — Java версия

import java.io.*;
import java.nio.file.*;
import java.security.*;
import java.util.Base64;

public class hash_generator {
    public static void main(String[] args) throws Exception {
        String algo = "SHA-512";
        String format = "hex";
        String filePath = null;
        String output = null;
        String compare = null;
        String input = null;

        for (int i = 0; i < args.length; i++) {
            switch (args[i]) {
                case "--algo":
                case "-a":
                    algo = mapAlgo(args[++i]);
                    break;
                case "--format":
                case "-f":
                    format = args[++i];
                    break;
                case "--file":
                case "-F":
                    filePath = args[++i];
                    break;
                case "--output":
                case "-o":
                    output = args[++i];
                    break;
                case "--compare":
                case "-c":
                    compare = args[++i];
                    break;
                default:
                    if (!args[i].startsWith("-")) {
                        input = args[i];
                    }
            }
        }

        System.out.println("\u001B[36m🔐 Hash Generator (Java)\u001B[0m");
        System.out.println("Алгоритм: " + algo);

        String hexDigest = "";
        if (filePath != null) {
            System.out.println("📂 Хеширование файла: " + filePath);
            hexDigest = hashFile(filePath, algo);
        } else if (input != null) {
            System.out.println("📝 Входные данные: " + input);
            hexDigest = computeHash(input.getBytes(), algo);
        } else {
            // stdin
            System.out.println("📝 Чтение из STDIN (Ctrl+D для окончания)");
            StringBuilder sb = new StringBuilder();
            BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
            String line;
            while ((line = reader.readLine()) != null) {
                sb.append(line).append("\n");
            }
            String data = sb.toString();
            if (data.isEmpty()) {
                System.out.println("\u001B[33m⚠️ Пустой ввод.\u001B[0m");
                System.exit(1);
            }
            hexDigest = computeHash(data.getBytes(), algo);
        }

        String result;
        if (format.equals("base64")) {
            byte[] bytes = hexStringToByteArray(hexDigest);
            result = Base64.getEncoder().encodeToString(bytes);
        } else {
            result = hexDigest;
        }

        System.out.println("\u001B[32mХеш (" + format + "):\u001B[0m");
        System.out.println(result);

        if (compare != null) {
            if (result.equals(compare)) {
                System.out.println("\u001B[32m✅ Хеши совпадают!\u001B[0m");
            } else {
                System.out.println("\u001B[31m❌ Хеши не совпадают!\u001B[0m");
            }
        }

        if (output != null) {
            try (FileWriter fw = new FileWriter(output)) {
                fw.write(result + "\n");
                System.out.println("\u001B[32m💾 Сохранено в " + output + "\u001B[0m");
            } catch (IOException e) {
                System.out.println("\u001B[31m❌ Ошибка сохранения: " + e.getMessage() + "\u001B[0m");
            }
        }
    }

    private static String mapAlgo(String algo) {
        switch (algo.toLowerCase()) {
            case "sha512": return "SHA-512";
            case "sha256": return "SHA-256";
            case "sha384": return "SHA-384";
            case "sha1": return "SHA-1";
            case "md5": return "MD5";
            default: return "SHA-512";
        }
    }

    private static String computeHash(byte[] data, String algo) throws NoSuchAlgorithmException {
        MessageDigest md = MessageDigest.getInstance(algo);
        byte[] digest = md.digest(data);
        return bytesToHex(digest);
    }

    private static String hashFile(String filename, String algo) throws Exception {
        MessageDigest md = MessageDigest.getInstance(algo);
        try (FileInputStream fis = new FileInputStream(filename)) {
            byte[] buffer = new byte[8192];
            int bytesRead;
            long total = new File(filename).length();
            long processed = 0;
            while ((bytesRead = fis.read(buffer)) != -1) {
                md.update(buffer, 0, bytesRead);
                processed += bytesRead;
                if (total > 1024 * 1024) {
                    double percent = (double) processed / total * 100;
                    System.err.printf("\r⏳ Прогресс: %.1f%%", percent);
                }
            }
            if (total > 1024 * 1024) System.err.println();
        }
        return bytesToHex(md.digest());
    }

    private static String bytesToHex(byte[] bytes) {
        StringBuilder sb = new StringBuilder();
        for (byte b : bytes) {
            sb.append(String.format("%02x", b));
        }
        return sb.toString();
    }

    private static byte[] hexStringToByteArray(String s) {
        int len = s.length();
        byte[] data = new byte[len / 2];
        for (int i = 0; i < len; i += 2) {
            data[i / 2] = (byte) ((Character.digit(s.charAt(i), 16) << 4)
                                 + Character.digit(s.charAt(i+1), 16));
        }
        return data;
    }
}
