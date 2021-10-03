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
        echom 'Usage: (QUOTE VALUE:ANY).'
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
        let l:value = s:findVar(l:content, a:ctx)
        if type(l:value) == v:t_number
            echom 'Found reference to an unbound variable: `' . l:content . '`.'
            return 0
        else
            return [l:value, a:ctx]
        endif
    elseif l:type == g:vinel_list_t
        if l:content[0]['type'] == g:vinel_symbol_t
            let l:name = l:content[0]['content']
            let l:args = l:content[1:]

            if l:name == 'DEFVAR'
                return vars#evalDefvar(l:args, a:ctx)
            elseif l:name == 'QUOTE'
                return s:evalQuote(l:args, a:ctx)
            elseif l:name == 'SETV'
                return vars#evalSetv(l:args, a:ctx)
            elseif l:name == 'PRINT'
                if len(l:args) == 1
                    let l:expr = eval#evalExpr(l:args[0], a:ctx)
                    if type(l:expr) == v:t_number
                        return 0
                    else
                        call Print(l:expr[0])
                        return l:expr
                    endif
                else
                    call eval#invalidArgc('PRINT', 1, l:args)
                    echom 'Usage: (PRINT E:ANY)'
                    return 0
                endif
            elseif l:name == 'LOOP'
                let l:ctx = a:ctx
                while 1
                    for expr in l:args
                        let l:ne = eval#evalExpr(expr, l:ctx)
                        if type(l:ne) == v:t_number
                            return 0
                        else
                            let l:ctx = l:ne[1]
                        endif
                    endfor
                endwhile
            elseif l:name == 'EVAL'
                if len(l:args) == 1
                    let l:expr = eval#evalExpr(l:args[0], a:ctx)
                    if type(l:expr) == v:t_number
                        return 0
                    else
                        return eval#evalExpr(l:expr[0], l:expr[1])
                    endif
                else
                    call eval#invalidArgc('EVAL', 1, l:args)
                    echo 'Usage: (EVAL E:ANY)'
                endif
            elseif l:name == 'READ'
                let l:raw = ''
                if len(l:args) == 0
                    let l:raw = input('')
                elseif len(l:args) == 1
                    let l:type = l:args[0]['type']
                    if l:type == g:vinel_string_t
                        let l:raw = l:args[0]['content']
                    else
                        echom 'Expected a string (e.g. `"Hello"`), found a ' . g:vinel_type_names[l:type]
                        echom 'Usage: (READ RAW:STRING?).'
                        return 0
                    endif
                else
                    call eval#invalidArgc('READ', 1, l:args)
                    echom 'Usage: (READ RAW:STRING?).'
                    return 0
                endif

                let l:e = reader#readExpr(l:raw, 0)
                if type(l:e) == v:t_number
                    return 0
                else
                    return [l:e[0], a:ctx]
                endif
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

function! eval#evalWithCtx(exprs, ctx) abort
    let l:ctx = a:ctx
    let l:res = []
    for expr in a:exprs
        let l:tmp = eval#evalExpr(expr, ctx)
        if type(l:tmp) == v:t_number
            return 0
        endif
        let l:res = l:tmp[0]
        let l:ctx = l:tmp[1]
    endfor
    return [l:res, l:ctx]
endfunction

function! eval#eval(exprs) abort
    return eval#evalWithCtx(a:exprs, [{}])
endfunction
