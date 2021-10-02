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

function! eval#invalidArgc(fn, argc, argv)
    echom 'Function `' . a:fn . '` takes ' . a:argc . ' arguments, but ' . len(a:argv) . ' arguments were supplied.'
endfunction

function! s:evalQuote(argv, ctx)
    if len(a:argv) != 1
        s:invalidArgc('QUOTE', 1, a:argv)
        echom 'Usage: (QUOTE VALUE(Any)).'
        return 0
    endif
    return [a:argv[0], a:ctx]
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
            if l:name == 'DEFVAR'
                return vars#evalDefvar(l:args, a:ctx)
            elseif l:name == 'QUOTE'
                return s:evalQuote(l:args, a:ctx)
            elseif l:name == 'QUOTE'
                return vars#evalSetv(l:args, a:ctx)
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
