Commands Reference
==================

This file will contain all the Vim Lisp commands reference, it means a description, a syntax explanation and a status (implemented / unimplemented).

map
---

![Status](https://img.shields.io/badge/Status-Implemented-success)

### Description

Map a sequence of keys to an action.

### Syntax

```scheme
(map
	keys: String ;; The keys sequence
	action*: Command 						;; The action(s) that will be triggered
	('normal | 'insert | 'visual)? 					;; The mode when the mapping should be active 
									;; (defaults to all modes)
	('buffer, 'nowait, 'silent, 'special, 'script, 'expr, 'unique)? ;; Special arguments 
	'recursive? 							;; Specify to enable recurson for the command
	)
```

let
---

![Status](https://img.shields.io/badge/Status-Implemented-success)

### Description

Declare a variable.

### Syntax

```scheme
(let 
	name: Var 						   ;; The variable name
	value: Expression 					   ;; The variable value
	('global | 'script | 'window | 'tab | 'buffer | 'function) ;; The variable scope
	)
```

get
---

![Status](https://img.shields.io/badge/Status-Implemented-success)

### Description

Get a property value.

### Syntax

```scheme
(get
	option?: Var       ;; The option to get (if not set, we will refer to the second parameter).	
	('all | 'termcap)? ;; Options to differ value display (not used if `option` is present).
	'newline?          ;; Specifies to display a newline after each option
)
```

set
---

![Status](https://img.shields.io/badge/Status-Implemented-success)

## Description

Set a property value.

### Syntax

```scheme
(set
	option: Var 				;; The option to set
	('toggle | 'reset | 'off | 'vi | 'vim)? ;; The change to apply
	value?: Expr 				;; The value to associate to the expression	
	)
```
