var N=null,E="",T="t",U="u",searchIndex={};
var R=["option","string","result","Asciis"];

searchIndex["asciis"]={"doc":"ASCII base on RFC20.","i":[[0,"asc","asciis",E,N,N],[3,R[3],"asciis::asc",E,N,N],[11,"ord",E,E,0,[[["self"],["str"]],[[R[0],["i32"]],["i32"]]]],[11,"chr",E,E,0,[[["self"],["i32"]],[[R[0],[R[1]]],[R[1]]]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[],[U]]],[11,"try_from",E,E,0,[[[U]],[R[2]]]],[11,"try_into",E,E,0,[[],[R[2]]]],[11,"borrow_mut",E,E,0,[[["self"]],[T]]],[11,"borrow",E,E,0,[[["self"]],[T]]],[11,"type_id",E,E,0,[[["self"]],["typeid"]]]],"p":[[3,R[3]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);