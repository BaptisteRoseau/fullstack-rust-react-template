{
  lib,
  fetchurl,
  rustPlatform,
  pkg-config,
  openssl,
}:

rustPlatform.buildRustPackage rec {
  pname = "sql-gen";
  version = "0.2.3";

  src = fetchurl {
    url = "https://static.crates.io/crates/${pname}/${pname}-${version}.crate";
    name = "${pname}-${version}.tar.gz";
    hash = "sha256-CGweGllsGfK+ib9Td2r/DC19LWxShKEoudeaM7RkC4k=";
  };

  cargoLock.lockFile = ./sql-gen-Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  doCheck = false;

  meta = {
    description = "CLI tool generating Rust models from a SQL database using SQLx";
    homepage = "https://github.com/jayy-lmao/sql-gen";
    license = lib.licenses.mit;
    mainProgram = "sql-gen";
  };
}
