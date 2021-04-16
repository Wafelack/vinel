Commands Reference
==================

This file will contain all the Vim Lisp commands reference, it means a description, a syntax explanation and a status (implemented / unimplemented).

map
---

![Status](https://img.shields.io/badge/Status-Implemented-sucess)

### Description

Map a sequence of keys to an action.

### Syntax

```scheme
(map
	keys: String ;; The keys sequence
	action*: Command ;; The action(s) that will be triggered
	('normal | 'insert | 'visual)? ;; The mode when the mapping should be active (defaults to all modes)
	('buffer, 'nowait, 'silent, 'special, 'script, 'expr, 'unique)? ;; Special arguments 
	'recursive? ;; Specify to enable recurson for the command
	)
```

let
---

![Status](https://img.shields.io/badge/Status-Implemented-sucess)

### Description

Declare a variable.

### Syntax

```scheme
(let 
	name: Var ;; The variable name
	value: Expression ;; The variable value
	('global | 'script | 'window | 'tab | 'buffer | 'function) ;; The variable scope
	)
```
