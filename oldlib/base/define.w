
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

[
    quote quote uplevel
    quote quote uplevel
    swap
    uplevel-in-named-context
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

