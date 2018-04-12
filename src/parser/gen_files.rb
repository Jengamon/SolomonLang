require "liquid"
require 'yaml'

tmpl_path, data_path, target = ARGV

if !tmpl_path.is_a? String or !data_path.is_a? String or !target.is_a? String then
    raise "Must supply three arguments"
end

def quick_read (file, prefix="")
  File.open(file, "r") { |b| b.readlines.map{|e| prefix + e}.join }
end

tmpl = quick_read "#{tmpl_path}.liq"
data = YAML.load_file "#{data_path}.yml"

puts tmpl
puts data

@template = Liquid::Template.parse(tmpl, :error_mode => :strict)
file = @template.render data


puts file

o = File.open("#{target}.rs", "w")
o << <<DOC
// Generated using gen_files.rb ver 0.0.1
// DO NOT MODIFY, AS IT WILL BE DELETED WHEN REGENERATED.

DOC
if File.exists?("../../LICENSE") then
  o << quick_read("../../LICENSE", "// ")
  o << "\n"
end
o << file
o.close
