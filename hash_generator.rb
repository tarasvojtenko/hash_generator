# hash_generator.rb — Ruby версия

require 'digest'
require 'optparse'
require 'base64'

options = {}
OptionParser.new do |opts|
  opts.banner = "Usage: ruby hash_generator.rb [options] [input]"
  opts.on("-a", "--algo ALGO", "Алгоритм (sha512, sha256, sha384, sha1, md5)") { |a| options[:algo] = a }
  opts.on("-f", "--format FORMAT", "Формат (hex, base64)") { |f| options[:format] = f }
  opts.on("-F", "--file FILE", "Файл для хеширования") { |f| options[:file] = f }
  opts.on("-o", "--output FILE", "Сохранить хеш в файл") { |o| options[:output] = o }
  opts.on("-c", "--compare HASH", "Сравнить с эталонным хешем") { |c| options[:compare] = c }
end.parse!

algo = options[:algo] || 'sha512'
format = options[:format] || 'hex'
file_path = options[:file]
output = options[:output]
compare = options[:compare]
input = ARGV[0]

puts "\e[36m🔐 Hash Generator (Ruby)\e[0m"
puts "Алгоритм: #{algo.upcase}"

def compute_hash(data, algo)
  case algo
  when 'sha512' then Digest::SHA512.hexdigest(data)
  when 'sha256' then Digest::SHA256.hexdigest(data)
  when 'sha384' then Digest::SHA384.hexdigest(data)
  when 'sha1'   then Digest::SHA1.hexdigest(data)
  when 'md5'    then Digest::MD5.hexdigest(data)
  else raise "Неподдерживаемый алгоритм"
  end
end

def hash_file(filename, algo)
  total = File.size(filename)
  processed = 0
  digest = case algo
  when 'sha512' then Digest::SHA512.new
  when 'sha256' then Digest::SHA256.new
  when 'sha384' then Digest::SHA384.new
  when 'sha1'   then Digest::SHA1.new
  when 'md5'    then Digest::MD5.new
  else raise "Неподдерживаемый алгоритм"
  end
  File.open(filename, 'rb') do |f|
    while chunk = f.read(8192)
      digest.update(chunk)
      processed += chunk.bytesize
      if total > 1024 * 1024
        percent = (processed.to_f / total) * 100
        STDERR.print "\r⏳ Прогресс: %.1f%%" % percent
      end
    end
  end
  STDERR.puts if total > 1024 * 1024
  digest.hexdigest
end

hex_digest = if file_path
  puts "📂 Хеширование файла: #{file_path}"
  hash_file(file_path, algo)
elsif input
  puts "📝 Входные данные: #{input}"
  compute_hash(input, algo)
else
  puts "📝 Чтение из STDIN (Ctrl+D для окончания)"
  data = STDIN.read
  if data.empty?
    puts "\e[33m⚠️ Пустой ввод.\e[0m"
    exit 1
  end
  compute_hash(data, algo)
end

result = if format == 'base64'
  Base64.strict_encode64([hex_digest].pack('H*'))
else
  hex_digest
end

puts "\e[32mХеш (#{format}):\e[0m"
puts result

if compare
  if result == compare
    puts "\e[32m✅ Хеши совпадают!\e[0m"
  else
    puts "\e[31m❌ Хеши не совпадают!\e[0m"
  end
end

if output
  File.write(output, result + "\n")
  puts "\e[32m💾 Сохранено в #{output}\e[0m"
end
