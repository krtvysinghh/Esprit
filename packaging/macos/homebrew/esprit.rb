class Esprit < Formula
  desc "AI-powered local knowledge engine"
  homepage "https://github.com/krtvysinghh/Esprit"
  url "https://github.com/krtvysinghh/Esprit/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_RELEASE_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "Esprit", shell_output("#{bin}/esprit --help")
  end
end
