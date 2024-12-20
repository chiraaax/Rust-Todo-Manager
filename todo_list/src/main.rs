use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use colored::*;

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    priority: String,
    category: String,
}
