extends Node2D

# --- Exports ----
@export var block_size_pixels = 32
@export var grid_offset = Vector2(50, 50)
@export var next_piece_offset = Vector2(400, 50)

var rust_game_node = null

# --- Drawing Data ---
# Dictionary to map BlockTypes to Godot Colors
var block_colors = {
	0: Color(0.1,0.1,0.1,1.0),    # background/empty dark grey
	1: Color(0.0, 1.0, 1.0, 1.0), # I - cyan
	2: Color(0.0, 0.0, 1.0, 1.0), # J - blue
	3: Color(1.0, 0.65, 0.0, 1.0),# L - orange
	4: Color(1.0, 1.0, 0.0, 1.0), # O - yellow
	5: Color(0.0, 1.0, 0.0, 1.0), # S - green
	6: Color(0.65, 0.0, 1.0, 1.0),# T - purple
	7: Color(1.0, 0.0, 0.0, 1.0), # Z - red
}

func _ready() -> void:
	rust_game_node = get_node("../TetrisGameNode")
	if rust_game_node == null:
		print("Error: TetrisGameNode not found!  Drawing will not work")
		set_process(false)
		return
		
	print("TetrisView script ready.  Found Rust Game Node")
	
	queue_redraw()
	
func _process(delta: float) -> void:
	print(delta)
	if rust_game_node != null:
		rust_game_node.process(delta)
	
	queue_redraw()
	
func _draw() -> void:
	if rust_game_node == null:
		return
	
	draw_grid()
	
	var piece_positions = rust_game_node.get_current_piece_positions()
	var piece_block_type = rust_game_node.get_current_piece_block_type()
	draw_piece(piece_positions, piece_block_type)
	
	piece_positions = rust_game_node.get_next_piece_positions()
	piece_block_type = rust_game_node.get_next_piece_block_type()
	draw_piece(piece_positions, piece_block_type)
			

func draw_grid() -> void:
	var grid_data = rust_game_node.get_grid_data();
	var grid_width = rust_game_node.get_grid_width();
	var grid_height = rust_game_node.get_grid_height();
	
	for y in range(grid_height):
		for x in range(grid_width):
			var block_type_id = grid_data[y * grid_width + x]
			var color = block_colors.get(block_type_id, Color(0, 0, 0, 0))
			var pixel_pos = grid_offset + Vector2(x, y) * block_size_pixels
			draw_rect(Rect2(pixel_pos, Vector2(block_size_pixels, block_size_pixels)), color)

func draw_piece(positions: Array[Vector2], block_type: int) -> void:
	if block_type != -1:
		var color = block_colors.get(block_type, Color(1, 1, 1, 1))
		for pos in positions:
			var pixel_pos = grid_offset + pos * block_size_pixels
			draw_rect(Rect2(pixel_pos, Vector2(block_size_pixels, block_size_pixels)), color)
