//! # POOP
//! This crate contains the libraries for parsing the language POOP.
//! Currently it only supports interpreting the language, but there are
//! plans to expand the capabilities to compile the language.

#![warn(missing_docs)]

extern crate llvm_sys as llvm;

pub mod codegen;
pub mod execution_engine;
pub mod lexer;
pub mod mir;
pub mod parser;
pub mod type_system;
