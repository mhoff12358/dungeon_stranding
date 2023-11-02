extends Control

signal instantiate_template(name: String, count: int)
signal update_template(previous: Control, name: String, count: int)

var name_label: Label;
var count_label: Label;

func _ready():
	var context: DiContext = DiContext.get_context(self)
	self.name_label = context.get_registered_node_with_id("Label", "name")
	self.count_label = context.get_registered_node_with_id("Label", "count")

	update_template.connect(self.apply_template_values)

func apply_template_values(_previous: Control, monster_name: String, monster_count: int):
	self.name_label.set_text(monster_name)
	self.count_label.set_text(str(monster_count))