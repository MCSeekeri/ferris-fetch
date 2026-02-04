{
  projectRootFile = "flake.nix";

  programs = {
    nixfmt = {
      enable = true;
      strict = true;
    };
    deadnix.enable = true;
    statix.enable = true;

    rustfmt.enable = true;
  };
}
