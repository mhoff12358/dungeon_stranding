extends OutOfDungeonViz

signal updated_state


func _ready():
    self.game_state().updated_state.connect(self.game_state_updated_state)


func game_state_updated_state():
    if self.is_out_of_dungeon():
        self.updated_state.emit()
