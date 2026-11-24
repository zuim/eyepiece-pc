#!/usr/bin/env ruby
require 'digest'
require 'fileutils'
require 'open3'

app = ARGV.fetch(0)
frameworks = File.join(app, 'Contents', 'Frameworks')
executables = %w[eyepiece-gui eyepiece-pc].map { |name| File.join(app, 'Contents', 'MacOS', name) }
libraries = {}
queue = []

def command(*args)
  output, error, status = Open3.capture3(*args)
  raise "#{args.join(' ')} failed: #{error}" unless status.success?
  output
end

def rpaths(file)
  lines = command('otool', '-l', file).lines
  lines.each_index.map do |index|
    next unless lines[index].strip == 'cmd LC_RPATH'
    lines[index + 1, 4].join[/\bpath (.+?) \(offset \d+\)/, 1]
  end.compact
end

def expand_path(path, loader, executable)
  path.sub('@loader_path', File.dirname(loader)).sub('@executable_path', File.dirname(executable))
end

def dependencies(file)
  command('otool', '-L', file).lines.drop(1).map do |line|
    line[/^\s+(.+?) \(compatibility version /, 1]
  end.compact
end

def system_library?(path)
  path.start_with?('/usr/lib/', '/System/Library/')
end

executables.each do |executable|
  queue << [executable, executable, executable, rpaths(executable)]
end

until queue.empty?
  file, source, executable, inherited_rpaths = queue.shift
  search_paths = rpaths(source) + inherited_rpaths
  dependencies(source).each do |dependency|
    next if system_library?(dependency)
    if source != executable && dependency == dependencies(source).first
      next
    end

    candidates = if dependency.start_with?('@rpath/')
      suffix = dependency.delete_prefix('@rpath/')
      search_paths.map { |path| File.join(expand_path(path, source, executable), suffix) }
    else
      [expand_path(dependency, source, executable)]
    end
    resolved = candidates.find { |candidate| File.file?(candidate) }
    raise "Cannot resolve #{dependency} referenced by #{source}" unless resolved
    resolved = File.realpath(resolved)
    next if system_library?(resolved)
    raise "Cannot bundle external framework #{resolved}" if resolved.include?('.framework/')

    unless libraries.key?(resolved)
      name = "#{Digest::SHA256.hexdigest(resolved)[0, 12]}-#{File.basename(resolved)}"
      destination = File.join(frameworks, name)
      raise "Library name collision: #{destination}" if File.exist?(destination)
      FileUtils.cp(resolved, destination)
      FileUtils.chmod('u+w', destination)
      command('install_name_tool', '-id', "@loader_path/#{name}", destination)
      libraries[resolved] = name
      queue << [destination, resolved, executable, search_paths]
    end
    prefix = file == executable ? '@executable_path/../Frameworks' : '@loader_path'
    command('install_name_tool', '-change', dependency, "#{prefix}/#{libraries.fetch(resolved)}", file)
  end
end

puts "Bundled #{libraries.length} non-system libraries"

(executables + libraries.values.map { |name| File.join(frameworks, name) }).each do |file|
  dependencies(file).each do |dependency|
    next if system_library?(dependency)
    resolved = expand_path(dependency, file, executables.first)
    raise "Unbundled dependency #{dependency} referenced by #{file}" unless File.file?(resolved) && File.realpath(resolved).start_with?(File.realpath(frameworks) + '/')
  end
end
