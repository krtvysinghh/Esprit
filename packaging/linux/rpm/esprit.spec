Name:           esprit
Version:        0.1.0
Release:        1%{?dist}
Summary:        AI-powered local knowledge engine
License:        MIT
URL:             https://github.com/krtvysinghh/Esprit
Source0:        https://github.com/krtvysinghh/Esprit/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

%description
AI-powered local knowledge engine.

%prep
%setup -q -n Esprit-%{version}

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
install -m 0755 target/release/esprit %{buildroot}%{_bindir}/esprit

%files
%{_bindir}/esprit
