package _go

type KModule struct {
	name  string
	items KModuleItem
}

type KModuleItem struct {
	itemType ItemType
}

type ItemType int32

const (
	func_type    ItemType = 0
	proto_type   ItemType = 1
	import_type  ItemType = 2
	export_type  ItemType = 3
	forward_type ItemType = 4
	data_type    ItemType = 5
	ref_data     ItemType = 6
	expr_data    ItemType = 7
)

type DataType int32

// K_T_I8
// K_T_U8
const (
	IntegerType DataType = 0
	FloatType   DataType = 1
	PointerType DataType = 2
	ReturnBlock DataType = 3
)

type KFunction struct {
	name string
}
