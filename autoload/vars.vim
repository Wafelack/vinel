"
" vinel.vim - VINEL Is Not Emacs Lisp
" Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
" 
" ------------------------------------------------------
" 
"    This file is part of VINEL.
" 
" VINEL is free software: you can redistribute it and/or modify
" it under the terms of the GNU General Public License as published by
" the Free Software Foundation, either version 3 of the License, or
" (at your option) any later version.
" 
" VINEL is distributed in the hope that it will be useful,
" but WITHOUT ANY WARRANTY; without even the implied warranty of
" MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
" GNU General Public License for more details.
" 
" You should have received a copy of the GNU General Public License
" along with VINEL.  If not, see <https://www.gnu.org/licenses/>.
" 
" File:       vars.vim
" Maintainer: Wafelack <wafelack@riseup.net>
" Version:    0.1.0

if exists('g:vinel_vars_loaded')
    finish
endif
let g:vinel_vars_loaded = 1

function! vars#evalDefvar(argv, ctx)
    if len(a:argv) != 2
        eval#invalidArgc('DEFVAR', 2, a:argv)
        echom 'Usage: (DEFVAR NAME(Symbol) VALUE(Any)).'
        return 0
    endif
    let l:name_type = a:argv[0]['type']
    if l:name_type != g:vinel_symbol_t
        echom 'Expected a symbol (e.g. `foo`) found a ' . g:vinel_type_names[l:name_type] . '.'
        echom 'Usage: (DEFVAR NAME(Symbol) VALUE(Any)).'
        return 0
    endif
    let l:ctx = a:ctx
    let l:name = a:argv[0]['content']
    let l:idx = len(l:ctx) - 1
    if has_key(l:ctx[l:idx], l:name)
        echom 'Attempted to redefine an existing variable: `' . l:name . '`.'
        return 0
    endif
    let l:ctx[l:idx][l:name] = a:argv[1]
    return [a:argv[0], l:ctx]
endfunction

function! vars#evalSetv(argv, ctx)
    if len(a:argv) != 2
        eval#invalidArgc("SETV", 2, a:argv)
        echom 'Usage: (SETV NAME(Symbol) VALUE(Any)).'
        return 0
    endif
    let l:name_type = a:argv[0]['type']
    if l:name_type != g:vinel_symbol_t
        echom 'Expected a symbol (e.g. `foo`) found a ' . g:vinel_type_names[l:name_type] . '.'
        echom 'Usage: (SETV NAME(Symbol) VALUE(Any)).'
        return 0
    endif
    let l:ctx = a:ctx
    let l:name = a:argv[0]['content']
    let l:counter = len(l:ctx) - 1
    for scope in reverse(l:ctx)
        if has_key(scope, l:name)
            let l:ctx[l:counter][l:name] = a:argv[1]
            return [a:argv[0], l:ctx]
        endif
        let l:counter -= 1
    endfor
    echom 'Attempted to set the value of a non-existing variable: `' . l:name . '`.'
    return 0
endfunction

