#[derive(PartialEq, Debug)] #[allow(dead_code)]
enum TerrainGround {
	Soil,
	Stone
}

#[derive(PartialEq, Debug)] #[allow(dead_code)]
enum TerrainBlock {
	Tree,
	Soil,
	Stone
}

#[derive(PartialEq, Debug)] #[allow(dead_code)]
enum Being {
	Orc,
	Human
}

struct Square {
	ground: TerrainGround,
	block: Option<TerrainBlock>,
	beings: Option<Being>
}

struct Grid {
	size: (usize, usize),
	squares: Vec<Square>
}

impl Grid {
	fn generate_empty(x:usize, y: usize) -> Grid {

		let total_squares = x * y;
		let mut squares: Vec<Square> = Vec::with_capacity(total_squares);
		for _ in 0..total_squares {
			squares.push(Square { ground: TerrainGround::Soil, block: None, beings: None });
		}

		Grid { size: (x,y), squares: squares}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_empty_grid() {
		let grid = ::Grid::generate_empty(5, 13);
		assert_eq!(grid.size, (5,13));
		let mut number_of_squares = 0;
		for square in &grid.squares {
			assert_eq!(square.ground, ::TerrainGround::Soil);
			assert_eq!(square.block, None);
			assert_eq!(square.beings, None);
			number_of_squares += 1;
		}

		assert_eq!(grid.squares.len(), 5*13);
		assert_eq!(number_of_squares, 5*13);
	}
}