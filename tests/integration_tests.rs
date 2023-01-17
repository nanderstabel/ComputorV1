mod integration_tests {
    use assert_cmd::prelude::*;
    use indoc::indoc;
    use std::process::Command;

    fn compare(input: &'static str, output: &'static str) {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg(input).assert().success().stdout(output);
    }

    #[test]
    fn subject_example1() {
        let input = "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0";
        let output = indoc!(
            "
			Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
			Polynomial degree: 2
			Discriminant is strictly positive, the two solutions are:
			0.905239
			-0.475131
		"
        );

        compare(&input, &output);
    }

    #[test]
    fn subject_example2() {
        let input = "5 * X^0 + 4 * X^1 = 4 * X^0";
        let output = indoc!(
            "
			Reduced form: 1 * X^0 + 4 * X^1 = 0
			Polynomial degree: 1
			The solution is:
			-0.250000
		"
        );

        compare(&input, &output);
    }

    #[test]
    fn subject_example3() {
        let input = "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0";
        let output = indoc!(
            "
			Reduced form: 5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0
			Polynomial degree: 3
			The polynomial degree is strictly greater than 2, I can't solve.
		"
        );

        compare(&input, &output);
    }

    #[test]
    fn subject_free_form_example1() {
        let input = "5*X^0+4*X^1-9.3*X^2=1*X^0";
        let output = indoc!(
            "
			Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0
			Polynomial degree: 2
			Discriminant is strictly positive, the two solutions are:
			0.905239
			-0.475131
		"
        );

        compare(&input, &output);
    }

    #[test]
    fn subject_free_form_example2() {
        let input = "5*X^0+4*X^1=4*X^0";
        let output = indoc!(
            "
			Reduced form: 1 * X^0 + 4 * X^1 = 0
			Polynomial degree: 1
			The solution is:
			-0.250000
		"
        );

        compare(&input, &output);
    }

    #[test]
    fn subject_free_form_example3() {
        let input = "8*X^0-6*X^1+0*X^2-5.6*X^3=3*X^0";
        let output = indoc!(
            "
			Reduced form: 5 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 0
			Polynomial degree: 3
			The polynomial degree is strictly greater than 2, I can't solve.
		"
        );

        compare(&input, &output);
    }
}
