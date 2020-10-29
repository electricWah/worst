
documentation [
    title "Generate a pico-8 cart"
    usage "pico-8 [ stuff... ]"
    example [
       "pico-8 {"
       "    cart \"testcart\""
       "    init {"
       "        \"Hello, world!\" print"
       "    }"
       "}"
    ]
    ; see-also pico-8/setup
    tags (pico-8)
]

define pico-8 [
    import meta/cil
    import data/map

    upquote const %p8body

    #f make-place const %cartname

    define init [
        upquote const %_initbody
        [ export-as "_init" define _init ]
        [] %_initbody list-push list-append
        eval
    ]

    define cart [ %cartname upquote place-set drop ]

    [
        import lua/pico-8/builtins
        %p8body
        cil/chunk->string
    ] eval
    const code

    %cartname place-get swap drop
    false? if [ drop "testcart" ] []
    ".p8" string-append
    const cart

    [
        cart open-output-file
        const current-output-port

        "pico-8 cartridge\n" print
        "version 29\n" print
        "__lua__\n" print
        code print
        "\n" print

        current-output-port port-close
    ] eval
]
export-name pico-8

doc-for pico-8/init [
    title "Init function"
    tags (pico-8)
]

doc-for pico-8/cart [
    title "Filename for cartridge"
    usage "cart \"filename\""
    tags (pico-8)
]

doc-for pico-8/update [
    title "Update function for pico-8 games"
    description "Code that is run once per update in a game"
    usage "pico-8 [ update [ code... ] ]"
    tags (pico-8)
]

; doc-for pico-8/stdlib [
;     title ""
; ]

; vi: ft=scheme

