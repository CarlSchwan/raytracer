/**
 * Copyright © 2019
 * Sami Shalayel <sami.shalayel@tutamail.com>,
 * Carl Schwan <carl@carlschwan.eu>,
 * Daniel Freiermuth <d_freiermu14@cs.uni-kl.de>
 *
 * This work is free. You can redistribute it and/or modify it under the
 * terms of the Do What The Fuck You Want To Public License, Version 2,
 * as published by Sam Hocevar. See the LICENSE file for more details.
 * 
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See the LICENSE
 * file for more details. **/

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;

#[derive(Debug)]
pub enum Error {
    ParseError(wavefront_obj::ParseError),
    Io(io::Error),
    Time(std::time::SystemTimeError),
    Error(String),
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref error) => error.fmt(formatter),
            Error::ParseError(ref error) => error.message.fmt(formatter),
            Error::Error(ref error) => error.fmt(formatter),
            Error::Time(ref error) => error.fmt(formatter),
        }
    }
}

impl From<wavefront_obj::ParseError> for Error {
    fn from(error: wavefront_obj::ParseError) -> Self {
        Error::ParseError(error)
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Error::Time(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Error(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Error::Error(String::from(error))
    }
}
