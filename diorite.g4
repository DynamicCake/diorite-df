grammar diorite;
// Note that this is just like more structured pseudo code and this isn't supposed be used by an
// antlr compiler

start: top_level*;
top_level: player_event | entity_event | func_def | proc_def;

player_event: 'pevent' IDEN stmt* 'end';
entity_event: 'eevent' IDEN stmt* 'end';
func_def: 'func' IDEN '(' arg_def_list? ')' stmt* 'end';
proc_def: 'proc' IDEN stmt* 'end';

stmt_type:
	'paction'
	| 'eaction'
	| 'gaction'
	| 'control'
	| 'callf'
	| 'callp'
	| 'select'
	| 'var';

stmt: reg_stmt | if_stmt | repeat_stmt;

reg_stmt:
	stmt_type IDEN ('<' IDEN '>')? ('[' iden_kv_list? ']')? (
		'(' expr_list? ')'
	);

if_stmt_type: 'ifplayer' | 'ifentity' | 'ifgame' | 'ifvar';

if_stmt:
	if_stmt_type 'not'? IDEN ('<' IDEN '>')? (
		'[' iden_kv_list? ']'
	)? ('(' expr_list? ')') stmt* ('end' | ('else' stmt* 'end')?); // No elif for you >:D

repeat_stmt:
	'repeat' IDEN ('<' IDEN '>')? ('[' iden_kv_list? ']')? (
		'(' expr_list? ')'
	) stmt* 'end';

arg_def_list: arg_def (',' arg_def)* ','?;
arg_def:
	// This second IDEN is the type of the param, these aren't keywords because they don't need to be differentianated
	IDEN ':' IDEN IDEN;

expr_list: expr (',' expr)* ','?;
expr:
	STRING
	| NUMBER
	| expr_lit_type '(' (STRING | NUMBER) (STRING | NUMBER)* ','? ')';
expr_lit_type:
	'svar'
	| 'gvar'
	| 'tvar'
	| 'lvar'
	| 'loc'
	| 'vec'
	| 'snd'
	| 'part'
	| 'pot'
	| 'gval'
	| 'text';

iden_kv_list: iden_kv_pair (',' iden_kv_pair)* ','?;
iden_kv_pair: (IDEN ':' IDEN);

// paction SendMessage <selection> ['Alignment Mode': 'Centered'] ("Hello Joj")
WS: [ \n\t\r]+ -> skip;
BLOCK_COMMENT: '/*' .*? '*/' -> skip;
EOL_COMMENT: '//' ~[\r\n]* -> skip;
STRING: '"' ~["]*? '"';
TEXT: '$' '"' ~["]*? '"';
IDEN: [a-zA-Z_]([a-zA-Z0-9_]*)? | '\'' ~[']*? '\'';
NUMBER: [0-9]+ ('.' [0-9]+)?;
// This antlr grammar isnt actually going to be used, so this Iden thing dosen't really matter All
// you need to know is that iden and 'iden' are all good