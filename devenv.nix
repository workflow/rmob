{ pkgs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "rmob dev shell";

  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.openssl
    pkgs.pkg-config # for OpenSSL
  ];

  # https://devenv.sh/scripts/
  scripts.hello.exec = "echo hello from $GREET";

  enterShell = ''
    hello
    rustc --version
    cargo --version
  '';

  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.rust.channel = "stable";

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
