extends GameStateViz


var started = false

func _process(delta):
    if not self.started:
        self.updated_state.emit()
        self.started = true

