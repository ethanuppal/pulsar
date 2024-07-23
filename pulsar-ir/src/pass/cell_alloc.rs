//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{from_ast::AsGeneratorPool, visitor::Visitor};

pub struct CellAlloc;

impl<P: AsGeneratorPool> Visitor<P> for CellAlloc {}