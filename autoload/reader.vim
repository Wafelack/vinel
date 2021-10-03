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
" File:       reader.vim
" Maintainer: Wafelack <wafelack@riseup.net>
" Version:    0.1.0

if exists('g:vinel_reader_loaded')
    finish
endif
let g:vinel_reader_loaded = 1

let g:vinel_string_t = 0
let g:vinel_symbol_t = 1
let g:vinel_number_t = 2
let g:vinel_list_t   = 3

function! s:readNum(raw)
    let l:num = ''
    let l:raw = a:raw
    while l:raw[0] =~ '\d'
        let l:num = l:num . l:raw[0]
        let l:raw = l:raw[1:]
    endwhile
    return [{ 'type' : g:vinel_number_t, 'content' : str2nr(l:num) }, l:raw]
endfunction

function! s:readString(raw)
    let l:raw = a:raw[1:] " Consume opening quote
    let l:content = ''
    while l:raw[0] != '"' && strlen(l:raw) != 0
        let l:content = l:content . l:raw[0]
        let l:raw = l:raw[1:]
    endwhile
    if strlen(l:raw) == 0
        echom 'Unfinished string at `' . a:raw[0:strlen(l:content)] . '`.'
        return 0
    else
        return [{ 'type' : g:vinel_string_t, 'content' : l:content }, l:raw[1:]]
    endif
endfunction

function! s:readSymbol(raw)
    let l:content = ''
    let l:raw = a:raw
    let l:terminating = [' ', '(', "'", ')', '\r', '\n', '\t', '`', ',', '"']
    while index(l:terminating, l:raw[0]) == -1 && strlen(l:raw) != 0
        let l:content = l:content . l:raw[0]
        let l:raw = l:raw[1:]
    endwhile
    return [s:makeSymbol(l:content), l:raw]
endfunction

function! s:readList(raw, inqq)
    let l:raw = a:raw[1:] " Consume opening parenthese
    let l:content = []
    if a:inqq
        let l:content = [s:makeSymbol('list')]
    endif
    while l:raw[0] != ')' && strlen(l:raw) != 0
        let l:quoted = 1
        if l:raw[0] == ','
            if a:inqq
                let l:raw = l:raw[1:]
                let l:quoted = 0
            else
                echom 'Unexpected `,` outside a quasi quote at `' . a:raw[0:strlen(a:raw) - strlen(l:raw)] . '`.'
                return 0
            endif
        endif
        let l:expr = reader#readExpr(l:raw, a:inqq)
        if type(l:expr) == v:t_number
            return 0
        elseif type(l:expr[0]) != v:t_list
            call add(l:content, a:inqq && l:quoted ? s:makeList([s:makeSymbol('quote'), l:expr[0]]) : l:expr[0])
        endif
        let l:raw = l:expr[1]
    endwhile

    if strlen(l:raw) == 0
        echom 'Unclosed parenthese at `' . a:raw[0:strlen(a:raw) - strlen(l:raw)] . '`.'
        return 0
    else
        return len(l:content) == 0 ? [s:makeSymbol('nil'), l:raw[1:]] : [s:makeList(l:content), l:raw[1:]]
    endif
endfunction

function! s:makeList(content)
    return { 'type' : g:vinel_list_t, 'content' : a:content }
endfunction

function! s:makeSymbol(content)
    return { 'type' : g:vinel_symbol_t, 'content' : toupper(a:content) }
endfunction

function! reader#readExpr(raw, inqq)
    let l:first = a:raw[0]
    if l:first =~ '\d'
        return s:readNum(a:raw)
    elseif l:first == "'"
        let l:expr = reader#readExpr(a:raw[1:], 0)
        return type(l:expr) == v:t_number ? 0 : [s:makeList([s:makeSymbol('quote'), l:expr[0]]), l:expr[1]]
    elseif l:first == '`'
        let l:expr = reader#readExpr(a:raw[1:], 1)
        return type(l:expr) == v:t_number ? 0 : [l:expr[0], l:expr[1]]
    elseif l:first == '"'
        return s:readString(a:raw)
    elseif l:first == ' ' || l:first == '\t' || l:first == '\n' || l:first == '\r'
        return [[], a:raw[1:]]
    elseif l:first == '('
        return s:readList(a:raw, a:inqq)
    else
        return s:readSymbol(a:raw)
    endif
endfunction

function! reader#read(input)
    let l:exprs = []
    let l:input = a:input
    while strlen(l:input) > 0
        if l:input[0] == ';'
            while l:input[0] != '\n' && strlen(l:input) > 0
                let l:input = l:input[1:]
            endwhile
        else
            let l:value = reader#readExpr(l:input, 0)
            if type(l:value) == v:t_number
                return l:value
            endif
            if len(l:value[0]) != 0
                let l:exprs = add(l:exprs, l:value[0])
            endif
            let l:input = l:value[1]
        endif
    endwhile
    return l:exprs
endfunction
