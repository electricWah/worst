(lang dune 3.10)

(name worst)

(generate_opam_files true)

(source
 (gitlab worst-lang/worst))

(license LICENSE)

; (documentation https://url/to/documentation)

(package
 (name worst)
 (synopsis "The Worst programming language")
 (description "An interpreter for the Worst programming language")
 (depends ocaml dune))

