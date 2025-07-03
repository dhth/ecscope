mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ecscope lets you monitor AWS ECS resources from the terminal

    Usage: ecscope [OPTIONS] <COMMAND>

    Commands:
      deps      List ECS deployments
      profiles  Manage ecscope's profiles
      monitor   Open monitoring TUI
      help      Print this message or the help of the given subcommand(s)

    Options:
      -c, --config-dir <PATH>  Config directory (to override ecscope's default config directory)
          --debug              Output debug information without doing anything
      -h, --help               Print help

    ----- stderr -----
    ");
}
