extends Node

@onready var game_state: GameStateViz = get_parent()


func _input(event):
    if event is InputEventKey:
        var key_event = event as InputEventKey
        if key_event.pressed and not key_event.echo:
            if key_event.key_label == KEY_1:
                self.game_state.handle_input(1)
            if key_event.key_label == KEY_2:
                self.game_state.handle_input(2)
            if key_event.key_label == KEY_3:
                self.game_state.handle_input(3)
            if key_event.key_label == KEY_4:
                self.game_state.handle_input(4)
            if key_event.key_label == KEY_5:
                self.game_state.handle_input(5)
            if key_event.key_label == KEY_6:
                self.game_state.handle_input(6)
            if key_event.key_label == KEY_7:
                self.game_state.handle_input(7)
            if key_event.key_label == KEY_8:
                self.game_state.handle_input(8)
            if key_event.key_label == KEY_9:
                self.game_state.handle_input(9)
            if key_event.key_label == KEY_0:
                self.game_state.handle_input(10)
            if key_event.key_label == KEY_D:
                self.game_state.handle_input(11)
            if key_event.key_label == KEY_W:
                self.game_state.handle_input(12)
            if key_event.key_label == KEY_A:
                self.game_state.handle_input(13)
            if key_event.key_label == KEY_S:
                self.game_state.handle_input(14)
            if key_event.key_label == KEY_SPACE:
                self.game_state.handle_input(15)
