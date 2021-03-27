# frozen_string_literal: true

require_relative 'imago/ffi'
require_relative 'imago/version'

module Imago
  def self.hash_image(path)
    Imago::FFI.hash_image(path)
  end

  def self.distance(left, right)
    Imago::FFI.hamming_distance(left, right)
  end
end
