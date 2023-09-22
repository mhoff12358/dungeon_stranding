extends Control

@export var map: TileMap
@export var in_dungeon_viz: InDungeonViz


func _gui_input(event):
	if !in_dungeon_viz.map_controlable:
		return
	if event is InputEventMouseMotion:
		if event.button_mask & MOUSE_BUTTON_MASK_LEFT:
			self.map.position += event.relative
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			self.map.scale *= 1.02
		if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			self.map.scale /= 1.02
