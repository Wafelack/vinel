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
" File:       eval.vim
" Maintainer: Wafelack <wafelack@riseup.net>
" Version:    0.1.0

if exists('g:vinel_eval_loaded')
    finish
endif
let g:vinel_eval_loaded = 1

let g:vinel_func_t = 4

function! s:findVar(name, ctx)
    for scope in reverse(a:ctx)
        if has_key(scope, a:name)
            return scope[a:name]
        endif
    endfor
    return 0
endfunction

function! s:invalidArgc(fn, argc, argv)
    echom 'Function `' . a:fn . '` takes ' . a:argc . ' arguments, but ' . len(a:argv) . ' arguments were supplied.'
endfunction

function! s:evalQuote(argv, ctx)
    if len(a:argv) != 1
        s:invalidArgc('quote', 1, a:argv)
        echom 'Usage: (quote VALUE(Any)).'
        return 0
    endif
    return [a:argv[0], a:ctx]
endfunction

function! s:evalDefvar(argv, ctx)
    if len(a:argv) != 2
        s:invalidArgc('defvar', 2, a:argv)
        echom 'Usage: (defvar NAME(Symbol) VALUE(Any)).'
        return 0
    endif
    let l:name_type = a:argv[0]['type']
    if l:name_type != g:vinel_symbol_t
        echom 'Expected a symbol (e.g. `foo`) found a ' . g:vinel_type_names[l:name_type] . '.'
        echom 'Usage: (defvar NAME(Symbol) VALUE(Any)).'
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

function! s:evalSetv(argv, ctx)
    if len(a:argv) != 2
        s:invalidArgc("setv", 2, a:argv)
        echom 'Usage: (setv NAME(Symbol) VALUE(Any)).'
        return 0
    endif
    let l:name_type = a:argv[0]['type']
    if l:name_type != g:vinel_symbol_t
        echom 'Expected a symbol (e.g. `foo`) found a ' . g:vinel_type_names[l:name_type] . '.'
        echom 'Usage: (setv NAME(Symbol) VALUE(Any)).'
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

function! eval#evalExpr(expr, ctx)
    let l:type = a:expr['type']
    let l:content = a:expr['content']
    if l:type == g:vinel_string_t || l:type == g:vinel_number_t
        return [a:expr, a:ctx]
    elseif l:type == g:vinel_symbol_t
        let s:value = s:findVar(l:content, a:ctx)
        if type(s:value) == v:t_number
            echom 'Found reference to an unbound variable: `' . l:content . '`.'
            return 0
        else
            return s:value
        endif
    elseif l:type == g:vinel_list_t
        if l:content[0]['type'] == g:vinel_symbol_t
            let l:name = l:content[0]['content']
            let l:args = l:content[1:]
            if l:name == 'defvar'
                return s:evalDefvar(l:args, a:ctx)
            elseif l:name == 'quote'
                return s:evalQuote(l:args, a:ctx)
            elseif l:name == 'setv'
                return s:evalSetv(l:args, a:ctx)
            else
                let l:fun = eval#evalExpr(l:content[0], a:ctx)
                echom l:fun
            endif
        else
            let l:fun = eval#evalExpr(l:content[0], a:ctx)
            echom l:fun
        endif
    endif
endfunction

function! eval#evalWithCtx(exprs, ctx)
    let l:ctx = a:ctx
    let l:res = []
    for expr in a:exprs
        let l:tmp = eval#evalExpr(exprs, ctx)
        if l:tmp == 0
            return 0
        endif
        let l:res = l:tmp[0]
        let l:ctx = l:tmp[1]
    endfor
    return [l:res, l:ctx]
endfunction

function! eval#eval(exprs)
    return eval#evalWithCtx(exprs, [{}])
endfunction
