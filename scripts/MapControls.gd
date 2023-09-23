extends Control

@export var map: TileMap
@export var in_dungeon_viz: InDungeonViz
@onready var game_state: GameStateViz = in_dungeon_viz.game_state()

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

func _unhandled_key_input(event):
	if in_dungeon_viz.is_moving_in_dungeon():
		if event.pressed and not event.echo:
			if event.key_label == KEY_1:
				self.game_state.handle_input(1)
			if event.key_label == KEY_2:
				self.game_state.handle_input(2)
			if event.key_label == KEY_3:
				self.game_state.handle_input(3)
			if event.key_label == KEY_4:
				self.game_state.handle_input(4)
			if event.key_label == KEY_5:
				self.game_state.handle_input(5)
			if event.key_label == KEY_6:
				self.game_state.handle_input(6)
			if event.key_label == KEY_7:
				self.game_state.handle_input(7)
			if event.key_label == KEY_8:
				self.game_state.handle_input(8)
			if event.key_label == KEY_9:
				self.game_state.handle_input(9)
			if event.key_label == KEY_0:
				self.game_state.handle_input(10)
			if event.key_label == KEY_D:
				self.game_state.handle_input(11)
			if event.key_label == KEY_W:
				self.game_state.handle_input(12)
			if event.key_label == KEY_A:
				self.game_state.handle_input(13)
			if event.key_label == KEY_S:
				self.game_state.handle_input(14)
			if event.key_label == KEY_SPACE:
				self.game_state.handle_input(15)
			if event.key_label == KEY_Q:
				self.game_state.handle_input(16)


