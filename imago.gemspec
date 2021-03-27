require_relative 'lib/imago/version'

Gem::Specification.new do |spec|
  spec.name          = "imago"
  spec.version       = Imago::VERSION
  spec.author        = ["andrewcameronsims"]
  spec.summary       = "Perceptual hashing of images with img_hash"
  spec.files         = Dir['lib/**/*', 'src/**/*.rs', 'Cargo.toml', 'LICENSE', 'README.md']
  spec.require_paths = ["lib"]
  spec.add_dependency 'ffi'
end
