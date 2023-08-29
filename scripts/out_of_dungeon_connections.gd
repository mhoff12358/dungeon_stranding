extends OutOfDungeonViz

signal updated_state()

func _ready():
    get_parent().updated_state.connect(self.game_state_updated_state)

func game_state_updated_state():
    if self.is_out_of_dungeon():
        self.updated_state.emit()
