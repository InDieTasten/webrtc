use super::*;

mod basic {
    use super::*;

    mod zero_distance {
        use super::*;

        generate_value_constraint_tests!(
            tests: [
                {
                    name: i64_setting,
                    settings: i64 => &[Some(42)],
                },
                {
                    name: f64_setting,
                    settings: f64 => &[Some(42.0)],
                },
            ],
            constraints: f64 => &[
                ValueSequenceConstraint {
                    exact: None,
                    ideal: None,
                },
                ValueSequenceConstraint {
                    exact: None,
                    ideal: Some(vec![42.0]),
                },
            ],
            expected: Ok(0.0)
        );
    }

    mod fract_distance {
        use super::*;

        #[test]
        fn i64_setting() {
            test_value_constraint!(
                checks: [
                    {
                        setting: i64 => Some(1),
                        constraint: f64 => ValueSequenceConstraint {
                            exact: None,
                            ideal: Some(vec![4.0]),
                        },
                        expected: Ok(0.75),
                    },
                    {
                        setting: i64 => Some(2),
                        constraint: f64 => ValueSequenceConstraint {
                            exact: None,
                            ideal: Some(vec![4.0]),
                        },
                        expected: Ok(0.5),
                    },
                    {
                        setting: i64 => Some(3),
                        constraint: f64 => ValueSequenceConstraint {
                            exact: None,
                            ideal: Some(vec![4.0]),
                        },
                        expected: Ok(0.25),
                    },
                ],
                validate: |actual, expected| {
                    assert_eq!(actual, expected);
                }
            );
        }

        #[test]
        fn f64_setting() {
            test_value_constraint!(
                checks: [
                    {
                        setting: f64 => Some(1.0),
                        constraint: f64 => ValueSequenceConstraint {
                            exact: None,
                            ideal: Some(vec![4.0]),
                        },
                        expected: Ok(0.75),
                    },
                    {
                        setting: f64 => Some(2.0),
                        constraint: f64 => ValueSequenceConstraint {
                            exact: None,
                            ideal: Some(vec![4.0]),
                        },
                        expected: Ok(0.5),
                    },
                    {
                        setting: f64 => Some(3.0),
                        constraint: f64 => ValueSequenceConstraint {
                            exact: None,
                            ideal: Some(vec![4.0]),
                        },
                        expected: Ok(0.25),
                    },
                ],
                validate: |actual, expected| {
                    assert_eq!(actual, expected);
                }
            );
        }
    }

    mod one_distance {
        use super::*;

        generate_value_constraint_tests!(
            tests: [
                {
                    name: i64_setting,
                    settings: i64 => &[Some(0)],
                },
                {
                    name: f64_setting,
                    settings: f64 => &[Some(0.0)],
                },
            ],
            constraints: f64 => &[ValueSequenceConstraint {
                exact: None,
                ideal: Some(vec![42.0]),
            }],
            expected: Ok(1.0)
        );
    }
}

mod required {
    use super::*;

    mod zero_distance {
        use super::*;

        generate_value_constraint_tests!(
            tests: [
                {
                    name: i64_setting,
                    settings: i64 => &[Some(42)],
                },
                {
                    name: f64_setting,
                    settings: f64 => &[Some(42.0)],
                },
            ],
            constraints: f64 => &[ValueSequenceConstraint {
                exact: Some(vec![42.0]),
                ideal: None,
            }],
            expected: Ok(0.0)
        );
    }

    mod inf_distance {
        use super::*;

        mod missing {
            use super::*;

            generate_value_constraint_tests!(
                tests: [
                    {
                        name: i64_setting,
                        settings: i64 => &[None],
                    },
                    {
                        name: f64_setting,
                        settings: f64 => &[None],
                    },
                ],
                constraints: f64 => &[
                    ValueSequenceConstraint {
                        exact: Some(vec![42.0]),
                        ideal: None,
                    },
                    ValueSequenceConstraint {
                        exact: Some(vec![42.0]),
                        ideal: Some(vec![42.0]),
                    },
                ],
                expected: Err(SettingFitnessDistanceError::Missing)
            );
        }

        mod mismatch {
            use super::*;

            generate_value_constraint_tests!(
                tests: [
                    {
                        name: i64_setting,
                        settings: i64 => &[Some(0)],
                    },
                    {
                        name: f64_setting,
                        settings: f64 => &[Some(0.0)],
                    },
                ],
                constraints: f64 => &[
                    ValueSequenceConstraint {
                        exact: Some(vec![42.0]),
                        ideal: None,
                    },
                    ValueSequenceConstraint {
                        exact: Some(vec![42.0]),
                        ideal: Some(vec![42.0]),
                    },
                ],
                expected: Err(SettingFitnessDistanceError::Mismatch)
            );
        }
    }
}
