
#! [program ...] quote name %define !#
[ 1 dig list->definition 1 dig quote add-definition uplevel ]
list->definition quote %define
quote add-definition
quote uplevel uplevel

#! define name [program ...] !#
[ quote quote uplevel
  quote quote uplevel
  1 dig
  quote %define uplevel
] quote define %define

#! [code ...] eval -> code ... !#
#! symbol eval -> symbol call !#
[
    [
        list->definition eval-definition
    ] quote eval-list %define
    symbol?
    quote eval-list quote call
    2 dig
    quote swap call-when drop
    call
] quote eval
quote %define quote uplevel uplevel

[
    quote interpreter-context-name uplevel
    equal? 2 negate dig drop swap
    [ quote %uplevel/named quote uplevel uplevel ]
    [ drop uplevel ]
    2 dig quote swap call-when drop
    eval
] quote %uplevel/named
quote %define quote uplevel uplevel

[
    quote quote uplevel
    quote quote uplevel
    swap
    %uplevel/named
] quote uplevel/named
quote %define quote uplevel uplevel

define export-global [
    quote quote uplevel
    clone quote take-definition uplevel
    swap
    uplevel/named <root> add-definition
]

define %rename-definition [
    swap quote take-definition uplevel
    swap quote add-definition uplevel
]

export-global define
export-global export-global
export-global %rename-definition

