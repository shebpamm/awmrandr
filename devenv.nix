{ pkgs, ... }:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [
  ];

  languages.rust.enable = true;
}
