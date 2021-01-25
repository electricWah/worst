
map-empty make-place const %definition-stats
export-name %definition-stats

#t const %%profile-enabled
export-name %%profile-enabled

define profile-enable [
    upquote const %%profile-enabled
    quote %%profile-enabled definition-copy-up
]
export-name profile-enable

[] const %%profile-defaults
export-name %%profile-defaults

define profile-defaults [
    upquote const %%profile-defaults
    quote %%profile-defaults definition-copy-up
]
export-name profile-defaults

define write-profile-trace-enter-call [
    ; drop
    current-error-port swap port-write-string drop
]
export-name write-profile-trace-enter-call

define write-profile-trace-clock [
    ; drop
    ->string
    current-error-port swap port-write-string drop
]
export-name write-profile-trace-clock

; profile [
;   ; name testo
;   ; trace #t
;   ; count #t
; ]
; define testo [...]

; get or create timer for this based on name
; body wrapper:
; [
;   init
;   body updo eval
;   deinit
; ]
; init:
;   get current time
; deinit:
;   get current time
;   get timer
;   add 1 to count
;   add (time - prev time) to total
;   max time? etc

define-attribute profile [
    args [opts]
    before [
        %%profile-enabled if [
            import data/map
            const %name
            const %defbody

            %%profile-defaults opts list-append pairs->map

            quote name map-get swap drop
            false? if [ drop %name ] []
            const timer-name

            quote trace map-get swap drop
            false? if [ drop #f ] []
            const write-trace

            quote count map-get swap drop
            false? if [ drop #f ] []
            const log-counts

            drop

            log-counts if [
                %definition-stats
                place-get
                timer-name map-get swap drop
                false? if [
                    drop
                    [ count 0 total-time 0 ]
                    pairs->map make-place
                ] [ ]
                const timer-place
                timer-name timer-place map-set
                place-set drop
                timer-place
            ] [ #f ]
            const timer-place

            
            write-trace if [
                [ write-profile-trace-enter-call ]
                timer-name ->string list-push
            ] [[]]

            [ interpreter-cpu-time const %%t0 ]
            list-append

            [
                updo eval
                interpreter-cpu-time const %%t1
            ] %defbody list-push
            list-append
            
            log-counts if [
                [
                    place-get
                    quote count map-get 1 add map-set
                    quote total-time map-get
                    %%t1 %%t0 negate add add
                    map-set
                    place-set drop
                ]
                timer-place list-push
                list-append
            ] []
            
            write-trace if [
                [
                %%t1 %%t0 negate add
                write-profile-trace-clock
                ] list-append
            ] []

            %name
        ] []
    ]
]
export-name profile

define profile-stats [
    import data/map
    %definition-stats place-get swap drop
    [] swap
    map-iterate [
        place-get swap drop
        swap quote name swap map-set
        list-push
    ]
]

export-name profile-stats

define profile-stats-reset [ %definition-stats map-empty place-set drop ]
export-name profile-stats-reset

; vi: ft=scheme

