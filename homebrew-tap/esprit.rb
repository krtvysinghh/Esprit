class Esprit < Formula
  desc "Modular AI and filesystem intelligence workspace engine"
  homepage "https://github.com/krtvysinghh/Esprit"
  url "https://github.com/krtvysinghh/Esprit/releases/download/v0.1.0/esprit-v0.1.0-x86_64-apple-darwin.tar.gz"
  version "0.1.0"

  def install
    bin.install "esprit"
  end

  test do
    system "#{bin}/esprit", "--version"
  end
end
