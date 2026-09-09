{ pkgs }:

pkgs.writeShellApplication {
  name = "ohshittings-formatter";
  runtimeInputs = [
    pkgs.findutils
    pkgs.gnused
    pkgs.rustfmt
    pkgs.nixfmt
  ];
  text = ''
    set -eu

    targets=("$@")
    if [ "''${#targets[@]}" -eq 0 ]; then
      targets=(.)
    fi

    for target in "''${targets[@]}"; do
      if [ -d "$target" ]; then
        while IFS= read -r -d "" file; do
          case "$file" in
            *.rs)
              ${pkgs.rustfmt}/bin/rustfmt "$file"
              ;;
            *.nix)
              ${pkgs.nixfmt}/bin/nixfmt "$file"
              ;;
          esac
        done < <(find "$target" -type f \( -name '*.rs' -o -name '*.nix' \) -print0)
      elif [ -f "$target" ]; then
        case "$target" in
          *.rs)
            ${pkgs.rustfmt}/bin/rustfmt "$target"
            ;;
          *.nix)
            ${pkgs.nixfmt}/bin/nixfmt "$target"
            ;;
        esac
      fi
    done
  '';
}
