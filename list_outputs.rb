#!/usr/bin/ruby

require "pathname"

Dir["./outputs/*"].each do |v|
  path = Pathname.new(v)
  puts "![#{path.basename(path.extname())}](#{path.sub("./", "")})"
end
