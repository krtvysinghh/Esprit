class Esprit < Formula
  desc "Modular AI and filesystem intelligence workspace engine"
  homepage "https://github.com/krtvysinghh/Esprit"
  url "https://github.com/krtvysinghh/Esprit/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "c5312ff70b2940cf0e3b1be2ad0106900897c61f928203373830e4f9c2087107"
  version "0.1.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "apps/esprit-cli", "--root", prefix
  end

  test do
    system "#{bin}/esprit", "--version"
  end
end
