" vinel.vim - VINEL Is Not Emacs Lisp
" File:       reader.vim
" Maintainer: Wafelack <wafelack@riseup.net>
" Version:    0.1.0
" License:    GPL-3.0-or-later

let g:vinel_string_t = 0
let g:vinel_symbol_t = 1
let g:vinel_number_t = 2
let g:vinel_list_t   = 3

function! s:readNum(raw)
    let l:num = ""
    let l:raw = a:raw
    while l:raw[0] =~ '\d'
        let l:num = l:num . l:raw[0]
        let l:raw = l:raw[1:]
    endwhile
    return [{ 'type' : g:vinel_number_t, 'content' : str2nr(l:num) }, l:raw]
endfunction

function! s:readString(raw)
    let l:raw = a:raw[1:] " Consume opening quote
    let l:content = ""
    while l:raw[0] != '"' && strlen(l:raw) != 0
        let l:content = l:content . l:raw[0]
        let l:raw = l:raw[1:]
    endwhile
    if strlen(l:raw) == 0
        echom "Unfinished string at `" . a:raw[0:strlen(l:content)] . "`."
        return 0
    return [{ 'type' : g:vinel_string_t, 'content' : l:content }, l:raw[1:]]
endfunction

function! s:readSymbol(raw)
    let l:content = ""
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
    while l:raw[0] != ')' && strlen(l:raw) != 0
        let l:quoted = 1
        if l:raw[0] == ','
            if a:inqq
                let l:raw = l:raw[1:]
                let l:quoted = 0
            else
                echom "Unexpected `,` outside a quasi quote at `" . a:raw[0:strlen(a:raw) - strlen(l:raw)] . "`."
                return 0
            endif
        endif
        let l:expr = ReadExpr(l:raw, a:inqq)
        if type(l:expr) == v:t_number
            return 0
        elseif type(l:expr[0]) != v:t_list
            call add(l:content, a:inqq && l:quoted ? s:makeList([s:makeSymbol("quote"), l:expr[0]]) : l:expr[0])
        endif
        let l:raw = l:expr[1]
    endwhile

    if strlen(l:raw) == 0
        echom "Unclosed parenthese at `" . a:raw[0:strlen(a:raw) - strlen(l:raw)] . "`."
        return 0
    else
        return len(l:content) == 0 ? [s:makeSymbol("nil"), l:raw[1:]] : [s:makeList(l:content), l:raw[1:]]
    endif
endfunction

function! s:makeList(content)
    return { 'type' : g:vinel_list_t, 'content' : a:content }
endfunction

function! s:makeSymbol(content)
    return { 'type' : g:vinel_symbol_t, 'content' : a:content }
endfunction

function! ReadExpr(raw, inqq)
    let l:first = a:raw[0]
    if l:first =~ '\d'
        return s:readNum(a:raw)
    elseif l:first == "'"
        let l:expr = ReadExpr(a:raw[1:], 0)
        return type(l:expr) == v:t_number ? 0 : [s:makeList(["quote", l:expr[0]]), l:expr[1]]
    elseif l:first == "`"
        let l:expr = ReadExpr(a:raw[1:], 1)
        return type(l:expr) == v:t_number ? 0 : [l:expr[0], l:expr[1]]
    elseif l:first == '"'
        return s:readString(a:raw)
    elseif l:first == ' ' || l:first == "\t" || l:first == "\n" || l:first == "\r"
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
        let l:value = ReadExpr(l:input, 0)
        if type(l:value) == v:t_number
            return l:value
        endif
        if len(l:value[0]) != 0
            let l:exprs = add(l:exprs, l:value[0])
        endif
        let l:input = l:value[1]
    endwhile
    return l:exprs
endfunction
