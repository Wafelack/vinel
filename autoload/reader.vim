" vinel.vim - VINEL Is Not Emacs Lisp
" File:       reader.vim
" Maintainer: wafelack <wafelack@riseup.net>
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
    return strlen(l:raw) == 0 ? 0 : [{ 'type' : g:vinel_string_t, 'content' : l:content }, l:raw[1:]]
endfunction

function! s:readSymbol(raw)
    let l:content = ""
    let l:raw = a:raw
    let l:terminating = [' ', '(', "'", ')', '\r', '\n', '\t', '`', ',', '"']
    while index(l:terminating, l:raw[0]) == -1 && strlen(l:raw) != 0
        let l:content = l:content . l:raw[0]
        let l:raw = l:raw[1:]
    endwhile
    return [{ 'type' : g:vinel_symbol_t, 'content' : l:content }, l:raw]
endfunction

function! s:readList(raw)
    let l:raw = a:raw[1:] " Consume opening parenthese
    let l:content = []
    while l:raw[0] != ')' && strlen(l:raw) != 0
        let l:expr = s:readExpr(l:raw)
        if type(l:expr) == v:t_number
            return 0
        endif
        if type(l:expr[0]) != v:t_list
            call add(l:content, l:expr[0])
        endif
        let l:raw = l:expr[1]
    endwhile
    return strlen(l:raw) == 0 ? 0 : [{ 'type' : g:vinel_list_t, 'content' : l:content }, l:raw[1:]]
endfunction

function! s:readExpr(raw)
    let l:first = a:raw[0]
    if l:first =~ '\d'
        return s:readNum(a:raw)
    elseif l:first == "'"
        let l:expr = s:readExpr(a:raw[1:])
        return type(l:expr) == v:t_number ? 0 : [{ 'type' : g:vinel_list_t, 'content' : ["quote", l:expr[0]] }, l:expr[1]]
    elseif l:first == '"'
        return s:readString(a:raw)
    elseif l:first == ' ' || l:first == "\t" || l:first == "\n" || l:first == "\r"
        return [[], a:raw[1:]]
    elseif l:first == '('
        return s:readList(a:raw)
    else
        return s:readSymbol(a:raw)
    endif
endfunction

function! reader#read(input)
    let l:exprs = []
    let l:input = a:input
    while strlen(l:input) > 0
        let l:value = s:readExpr(l:input)
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
