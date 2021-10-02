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
" File:       printer.vim
" Maintainer: Wafelack <wafelack@riseup.net>
" Version:    0.1.0

if exists('g:vinel_printer_loaded')
    finish
endif
let g:vinel_printer_loaded = 1

function! Show(expr)
    let l:type = a:expr['type']
    let l:content = a:expr['content']
    
    if l:type == g:vinel_string_t
        return '"' . l:content . '"'
    elseif l:type == g:vinel_symbol_t || l:type == g:vinel_symbol_t
        return l:content
    elseif l:type == g:vinel_list_t
        let l:out = '( '
        for expr in l:content
            let l:out .= Show(expr) . ' '
        endfor
        return l:out . ')'
    elseif l:type == g:vinel_func_t
        return "#<function>"
    endif
endfunction

function! Print(expr)
    echom Show(a:expr)
endfunction
