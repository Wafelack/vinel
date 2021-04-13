Commands Reference
==================

This file will contain all the Vim Lisp commands reference, it means a description, a syntax explanation and a status (implemented / unimplemented).

map
---

- Status: Unimplemented.

### Description

Map a sequence of keys to an action.

### Syntax

```scheme
(map
	keys: String ;; The keys sequence
	action...: Command ;; The action(s) that will be triggered
	'normal | 'insert | 'visual | 'any ;; The mode when the mapping should be active
	'buffer, 'nowait, 'silent, 'special, 'script, 'expr, 'unique ;; Special arguments 
	'recursive ;; Specify to enable recurson for the command
	)
```
