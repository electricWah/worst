
; Command-line interface module: import this to enable "worst my-module"

; dynamics won't work because it crosses a module boundary (i.e. new interpreter)
#f make-place const current-cli-module
#f make-place const current-cli-arguments
#f make-place const cli-was-run

define cli-module-run [
    const module
    current-cli-module module place-set drop
    current-cli-arguments swap place-set drop
    module updo module-import
    cli-was-run place-get if [ ] [ module "not a cli module" error ]
]
export cli-module-run

define cli-run [
    upquote const run-body
    updo current-module
    false? if [ drop "cli-run: not in a module" error ] [
        module-imported-name const running-module
        current-cli-module place-get
        false? not if [
            running-module
            equal if [
                cli-was-run #t place-set drop
                current-cli-arguments place-get run-body
            ] [ [] ]
        ] [ [] ]
    ]
    updo eval
]
export cli-run

