mod common;
use common::{ExpectedFailure, ExpectedSuccess, Fixture};
use pretty_assertions::assert_eq;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn adding_a_profile_works() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["profiles", "add", "prof1"]);
    let mut cmd_two = fixture.command();
    cmd_two.args(["profiles", "add", "prof2"]);

    // WHEN
    let output = cmd.output().expect("command should've run");
    let output_two = cmd_two.output().expect("command two should've run");

    // THEN
    output.print_stderr_if_failed(None);
    assert!(output.status.success());
    output_two.print_stderr_if_failed(None);
    assert!(output_two.status.success());

    let mut list_cmd = fixture.command();
    list_cmd.args(["profiles", "list"]);
    let list_output = list_cmd.output().expect("list command should've run");
    assert!(list_output.status.success());
    let list_stdout = String::from_utf8(list_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(list_stdout.lines().count(), 2);
    assert!(list_stdout.contains("prof1"));
    assert!(list_stdout.contains("prof2"));
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn using_incorrect_profile_name_fails() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["profiles", "add", "split in three"]);

    // WHEN
    let output = cmd.output().expect("command should've run");

    // THEN
    output.print_stdout_if_succeeded(None);
    assert!(!output.status.success());
}

#[test]
fn adding_duplicate_profile_fails() {
    // GIVEN
    let fixture = Fixture::new();
    let mut cmd = fixture.command();
    cmd.args(["profiles", "add", "prof1"]);
    let mut cmd_two = fixture.command();
    cmd_two.args(["profiles", "add", "prof1"]);

    // WHEN
    let output = cmd.output().expect("command should've run");
    let output_two = cmd_two.output().expect("command two should've run");

    // THEN
    output.print_stderr_if_failed(None);
    assert!(output.status.success());
    output_two.print_stdout_if_succeeded(None);
    assert!(!output_two.status.success());
}
