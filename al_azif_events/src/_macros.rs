#[macro_export]
macro_rules! parse_comp_arg {
    ($arg:expr, $t:ty) => {
        $arg.parse::<$t>().map_err(|_| EventError::CouldNotParseComponentArgIntoType {
            arg:       FixedString::from_str_trunc($arg),
            into_type: stringify!($t),
        })
    };
}

#[macro_export]
macro_rules! parse_slash_arg {
    ($label:lifetime, $iter:expr, $name:expr, &str) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::String(value), .. } => *value,
                ResolvedOption { name: _, value: ResolvedValue::String(_), .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "String",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "String",
                    })
                },
                _ => break $label Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "String",
                }),
            }
        } else {
            break $label Err(EventError::MissingRequiredSlashCommandOption { name: $name })
        }
    };
    ($label:lifetime, $iter:expr, $name:expr, Option<str>) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::String(value), .. } => Some(*value),
                ResolvedOption { name: _, value: ResolvedValue::String(_), .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "String",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "String",
                    })
                },
                _ => break $label Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "String",
                }),
            }
        } else {
            None
        }
    };
    ($label:lifetime, $iter:expr, $name:expr, i64) => {
        if let Some(opt) = $iter.find(|opt| opt.name == $name) {
            match opt {
                ResolvedOption { name: $name, value: ResolvedValue::Integer(value), .. } => *value,
                ResolvedOption { name: _, value: ResolvedValue::Integer(_), .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionName {
                        r#type:        "Integer",
                        expected_name: $name,
                    })
                },
                ResolvedOption { name: $name, value: _, .. } => {
                    break $label Err(EventError::ExpectedAnotherSlashCommandOptionType {
                        name:          $name,
                        expected_type: "Integer",
                    })
                },
                _ => break $label Err(EventError::ExpectedAnotherSlashCommandOption {
                    expected_name: $name,
                    expected_type: "Integer",
                }),
            }
        } else {
            break $label Err(EventError::MissingRequiredSlashCommandOption { name: $name })
        }
    };
}
