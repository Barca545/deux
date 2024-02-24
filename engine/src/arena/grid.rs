use std::fmt::Display;
use crate::math::Vec3;
use serde::{Deserialize, Serialize};
use eyre::Result; 

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Terrain{
  #[default]
  Passable,
  Bush,
  Impassable
}

impl Display for Terrain{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.to_string())
  }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Grid{
  pub num_columns: usize,
  pub num_rows: usize,
  pub cell_size: f32,
  //unsure if the offsets should be pub 
  column_offset: i32,
  row_offset: i32,
  pub cells:Vec<Terrain>
}

impl Grid{
  ///Creates a new [`Grid`]. 
  /// `Grid` indexes start in the bottom right corner.
  pub fn new(width:u32, height:u32, cell_size:f32) -> Result<Self> {
    //Calculate the number of columns/rows 
    let num_columns = ((width as f32)/cell_size) as usize;
    let num_rows = ((height as f32)/cell_size) as usize;
    
    //Check to ensure the grid is divisible by two and the cell size is greater than 0
    assert!(num_columns % 2 == 0);
    assert!(num_rows % 2 == 0);
    assert!(cell_size > 0.0);

    //Calculate the offsets of the grid by dividing the number of rows and columns since have the grid is negative
    let column_offset = (num_columns/2) as i32;
    let row_offset = ((num_rows+1)/2) as i32;
    
    //Create a vector of Terrain initialized to passable containing exactly as many cells as the grid 
    let max_index = num_columns * num_rows;
    let cells = vec![Terrain::default(); max_index];
    
    Ok(Self{
      num_columns,
      num_rows,
      cell_size,
      column_offset,
      row_offset,
      cells
    })
  }
  ///Returns the index of a cell from a given position.
  /// The process the function uses is called a raster scan.
  /// Uses floor to calculate the x_index and z_index meaning it defaults towards the bottom-left square if on a cell boundry.
  pub fn get_cell_index(&self, position:Vec3) -> usize {
    //Offset the position values since the grid is centered at (0,0) instead of starting there
    let adjusted_x = position.x;
    let adjusted_z = position.z;

    //Calculate the row and column indices
    let mut column_index = (adjusted_x/self.cell_size).floor() as i32 + self.column_offset;
    let mut row_index = (adjusted_z/self.cell_size).floor() as i32 + self.row_offset;

    //If the index is greater than the max value set it to the max value
    //If the index is smaller than 0 value set it to 0
    if column_index >= self.num_columns as i32{
      column_index = self.num_columns as i32 -1;
    }
    if column_index < 0 {
      column_index = 0;
    }
    if row_index >= self.num_rows as i32{
      row_index = self.num_rows as i32 -1;
    }
    if row_index < 0 {
      row_index = 0;
    }

    //Calculate the cell index
    let cell_index = (row_index as usize * self.num_columns) + column_index as usize;
    cell_index
  }

  pub fn get_cell_terrain(&self, index:usize)->Terrain{
    self.cells[index]
  }

  ///Checks if a [`Grid`] cell is passable. 
  /// Returns true if the cell underlying the position does not contain `Impassable` [`Terrain`].
  pub fn is_passable(&self, position:Vec3) -> bool{
    let cell = self.get_cell_index(position);
      let terrain = self.get_cell_terrain(cell);
      match terrain {
        Terrain::Passable => true,
        Terrain::Bush => true,
        Terrain::Impassable => false
      } 
  }
}
