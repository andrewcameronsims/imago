require 'ffi'

module Imago
  module FFI
    extend ::FFI::Library
    lib_name = "libimago.#{::FFI::Platform::LIBSUFFIX}"
    ffi_lib File.expand_path(lib_name, __dir__)
    attach_function :hash_image, [:string], :string
    attach_function :hamming_distance, %i[string string], :uint
  end
end
