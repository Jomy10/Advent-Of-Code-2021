#!/usr/bin/env ruby
require 'beaver'

BUILD_DIR="out"
SRC_DIR="src"

CC_FLAGS="-g -Og -Wall -Werror=implicit"

CC = "clang"

system "mkdir -p out"

command :run do
  $beaver.call :build_obj
  $beaver.call :build
  sys "./#{BUILD_DIR}/main"
end

command :test do
  $beaver.call :build_obj
  $beaver.call :build_tests
  sys "./#{BUILD_DIR}/tests"
end

command :test_dbg do
  $beaver.call :build_obj
  $beaver.call :build_tests
  sys "lldb #{BUILD_DIR}/tests"
end

command :build_obj, src: "#{SRC_DIR}/*.c", target_dir: BUILD_DIR, target_ext: ".o" do |src, target|
  sys "#{CC} #{CC_FLAGS} #{src} -c -o #{target.gsub("src/", "")}"
end

command :build, src: "#{BUILD_DIR}/*.o", target: "#{BUILD_DIR}/main" do |srcs, target|
  sys "#{CC} #{CC_FLAGS} #{srcs.gsub("#{BUILD_DIR}/tests.o", "")} -o #{target}"
end

command :build_tests, src: "#{BUILD_DIR}/*.o", target: "#{BUILD_DIR}/tests" do |srcs, target|
  sys "#{CC} #{CC_FLAGS} #{srcs.gsub("#{BUILD_DIR}/main.o", "")} -o #{target}"
end

$beaver.end
