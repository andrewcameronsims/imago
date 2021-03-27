# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

task default: :spec

task :rust_build do
  `cargo rustc --release`
  `mv -f ./target/release/libimago.dylib ./lib/imago`
end

task build: :rust_build
task spec: :rust_build


task :rust_clean do
  `cargo clean`
  `rm -f ./lib/imago/libimago.dylib`
end

task clean: :rust_clean
