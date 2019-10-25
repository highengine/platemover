
// Uint8Raster represents a grid where each cell contains a 32 bit floating point value
// A Uint8Raster is composed of two parts:
//    The first is a object of type Grid, representing a collection of vertices that are connected by edges
//    The second is a typed array, representing a value for each vertex within the grid
// 
// Uint8Raster should theoretically work for any graph of vertices given the appropriate grid object,
// However tectonics.js only uses them with spherical grids.
// 
// Uint8Rasters can be viewed through several paradigms: vector calculus, math morphology, image editing, etc.
// Each paradigm has its own unique set of operations that it can perform on rasters objects.
// A developer needs to switch between paradigms effortlessly and efficiently, without type conversion.
// Rather than clutter the Uint8Raster class, operations on Uint8Rasters 
// are spread out as friend functions across several namespaces. Each namespace corresponds to a paradigm. 
// This design is meant to promote separation of concerns at the expense of encapsulation.
// I want raster objects to be as bare as possible, functioning more like primitive datatypes.

function Uint8Raster(grid, fill) {
  var result = new Uint8Array(grid.vertices.length);
  result.grid = grid;
  if (fill !== void 0) { 
  for (var i=0, li=result.length; i<li; ++i) {
      result[i] = fill;
  }
  }
  return result;
};
Uint8Raster.OfLength = function(length, grid) {
  var result = new Uint8Array(length);
  result.grid = grid;
  return result;
}
Uint8Raster.FromBuffer = function(buffer, grid) {
  var result = new Uint8Array(buffer, 0, grid.vertices.length);
  result.grid = grid;
  return result;
}
Uint8Raster.FromUint8Raster = function(raster) {
  var result = Uint8Raster(raster.grid);
  for (var i=0, li=result.length; i<li; ++i) {
      result[i] = raster[i];
  }
  return result;
}
Uint8Raster.FromUint16Raster = function(raster) {
  var result = Uint8Raster(raster.grid);
  for (var i=0, li=result.length; i<li; ++i) {
      result[i] = raster[i];
  }
  return result;
}
Uint8Raster.copy = function(raster, result) {
  var result = result || Uint8Raster(raster.grid);
  ASSERT_IS_ARRAY(raster, Uint8Array)
  ASSERT_IS_ARRAY(result, Uint8Array)
  result.set(raster);
  return result;
}
Uint8Raster.fill = function (raster, value) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  raster.fill(value);
};

Uint8Raster.min_id = function (raster) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  var max = Infinity;
  var max_id = 0;
  var value = 0;
  for (var i = 0, li = raster.length; i < li; i++) {
    value = raster[i];
    if (value < max) {
      max = value;
      max_id = i;
    };
  }
  return max_id;
};

Uint8Raster.max_id = function (raster) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  var max = -Infinity;
  var max_id = 0;
  var value = 0;
  for (var i = 0, li = raster.length; i < li; i++) {
    value = raster[i];
    if (value > max) {
      max = value;
      max_id = i;
    };
  }
  return max_id;
};

Uint8Raster.get_nearest_value = function(raster, pos) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  return raster[raster.grid.getNearestId(pos)];
}
Uint8Raster.get_nearest_values = function(value_raster, pos_raster, result) {
  result = result || Uint8Raster(pos_raster.grid);
  ASSERT_IS_ARRAY(value_raster, Uint8Array)
  ASSERT_IS_VECTOR_RASTER(pos_raster)
  ASSERT_IS_ARRAY(result, Uint8Array)
  var ids = pos_raster.grid.getNearestIds(pos_raster);
  for (var i=0, li=ids.length; i<li; ++i) {
      result[i] = value_raster[ids[i]];
  }
  return result;
}
Uint8Raster.get_ids = function(value_raster, id_array, result) {
  result = result || (id_array.grid !== void 0? Uint8Raster(id_array.grid) : Uint8Array(id_array.length));
  ASSERT_IS_ARRAY(value_raster, Uint8Array)
  ASSERT_IS_ARRAY(result, Uint8Array)
  for (var i=0, li=id_array.length; i<li; ++i) {
      result[i] = value_raster[id_array[i]];
  }
  return result;
}
Uint8Raster.get_mask = function(raster, mask) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  ASSERT_IS_ARRAY(mask, Uint8Array)
  var result = new Uint8Array(Uint8Dataset.sum(mask));
  for (var i = 0, j = 0, li = mask.length; i < li; i++) {
    if (mask[i] > 0) {
      result[j] = raster[i];
      j++;
    }
  }
  return result;
}
Uint8Raster.set_ids_to_value = function(raster, id_array, value) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  for (var i=0, li=id_array.length; i<li; ++i) {
      raster[id_array[i]] = value;
  }
  return raster;
}
Uint8Raster.set_ids_to_values = function(raster, id_array, value_array) {
  ASSERT_IS_ARRAY(raster, Uint8Array)
  for (var i=0, li=id_array.length; i<li; ++i) {
      raster[id_array[i]] = value_array[i];
  }
  return raster;
}