extends Control


# Called when the node enters the scene tree for the first time.
func _ready():
    self.get_parent().get_parent().updated_state.connect(game_state_updated_state)

func game_state_updated_state():
    self.text = self.get_parent().get_parent().get_tiles()
