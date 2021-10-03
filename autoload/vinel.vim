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
" File:       vinel.vim
" Maintainer: Wafelack <wafelack@riseup.net>
" Version:    0.1.0

if exists('g:vinel_loaded')
    finish
endif
let g:vinel_loaded = 1

let g:vinel_type_names = ["STRING", "SYMBOL", "NUMBER", "LIST", "FUNCTION"]

function! FullEvalLine(s, ctx) abort
    echom " "
    let l:exprs = reader#read(a:s)
    if type(l:exprs) != v:t_number
        let l:r = eval#evalWithCtx(l:exprs, a:ctx)
        if type(l:r) != v:t_number
            echom "=> " . Show(l:r[0])
            return l:r[1]
        endif
    endif
    return 0
endfunction

function! Repl() abort
    let l:ctx = [{}]
    let l:ln = 0
    while 1
        let l:ln += 1
        let l:s = input('VINEL-USER:' . l:ln . '> ')
        if len(l:s) != 0
            let l:out = FullEvalLine(l:s, l:ctx)
            if type(l:out) != v:t_number
                let l:ctx = l:out
            endif
        endif
    endwhile
endfunction

nnoremap <leader>x :call FullEvalLine(input('M-x '), [{}])<CR>
nnoremap <leader>r :call Repl()<CR>
