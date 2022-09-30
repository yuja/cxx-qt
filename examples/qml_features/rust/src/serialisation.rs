// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DataSerde {
    number: i32,
    string: String,
}

impl From<&Serialisation> for DataSerde {
    fn from(value: &Serialisation) -> DataSerde {
        DataSerde {
            number: value.number,
            string: value.string.to_string(),
        }
    }
}

#[cxx_qt::bridge(cxx_file_stem = "serialisation")]
mod ffi {
    use super::DataSerde;

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject]
    pub struct Serialisation {
        #[qproperty]
        pub number: i32,
        #[qproperty]
        pub string: QString,
    }

    impl Default for Serialisation {
        fn default() -> Self {
            let string = r#"{"number": 4, "string": "Hello World!"}"#;
            let data_serde: DataSerde = serde_json::from_str(string).unwrap();
            data_serde.into()
        }
    }

    impl From<DataSerde> for Serialisation {
        fn from(value: DataSerde) -> Serialisation {
            Serialisation {
                number: value.number,
                string: QString::from(&value.string),
            }
        }
    }

    impl qobject::Serialisation {
        #[qinvokable]
        pub fn as_json_str(&self) -> QString {
            let data_serde = DataSerde::from(self.rust());
            let data_string = serde_json::to_string(&data_serde).unwrap();
            QString::from(&data_string)
        }

        // ANCHOR: book_grab_values
        #[qinvokable]
        pub fn grab_values(mut self: Pin<&mut Self>) {
            let string = r#"{"number": 2, "string": "Goodbye!"}"#;
            let data_serde: DataSerde = serde_json::from_str(string).unwrap();
            self.as_mut().set_number(data_serde.number);
            self.as_mut().set_string(QString::from(&data_serde.string));
        }
        // ANCHOR_END: book_grab_values
    }
}
// ANCHOR_END: book_macro_code
