
;;; vi: ft=scheme

define-record-type* &type [name]
define-record-type* &literal [val]
define-record-type* &variable [name]
define-record-type* &funcall [name args]
define-record-type* &expr [val type]
define-record-type* &assign [name expr]
define-record-type* &function [name args body]



