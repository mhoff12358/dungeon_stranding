extends Label


func _on_out_of_dungeon_viz_updated_state():
	self.text = get_parent().shop_list()
