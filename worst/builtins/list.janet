
(import ../interpreter :as I)
(import ../data)

(defn list-length :builtin {:i [data/List] :o [data/I64]} [i l]
  [(data/list-length l)])

(defn list-append :builtin {:i [data/List data/List] :o [data/List]} [i a b]
  [(data/list-append a b)])

(defn list-pop :builtin {:i [data/List] :o [data/List :val]} [i l]
  (let [v (data/list-pop l)] [l (data/nil->err v)]))

(defn list-get :builtin {:i [data/List data/I64] :o [:val]} [i l idx]
  (let [idx (if (neg? idx) (+ (data/list-length l) idx) idx)]
    [(data/nil->err (data/list-get l idx))]))

(defn list-empty :builtin {:i [data/List] :o [:boolean]} [i l] [(data/list-empty? l)])

# /// list `list-length` -> i64 : the length of the list.
# pub fn list_length(i: &mut Interpreter) -> BuiltinRet {
#     let l = i.stack_pop::<List>()?;
#     i.stack_push(l.as_ref().len() as i64);
#     Ok(())
# }

# /// list val `list-push` -> list : put the value at the front of the list.
# pub fn list_push(i: &mut Interpreter) -> BuiltinRet {
#     let v = i.stack_pop_val()?;
#     let mut l = i.stack_pop::<List>()?;
#     l.as_mut().push(v);
#     i.stack_push(l);
#     Ok(())
# }

# /// list `list-pop` +-> val : take the front value off the list (or false).
# pub fn list_pop(i: &mut Interpreter) -> BuiltinRet {
#     let mut l = i.stack_pop::<List>()?;
#     let v = l.as_mut().pop().unwrap_or_else(|| false.into());
#     i.stack_push(l);
#     i.stack_push(v);
#     Ok(())
# }

# /// list `list-reverse` -> list : reverse the list.
# pub fn list_reverse(i: &mut Interpreter) -> BuiltinRet {
#     let mut l = i.stack_pop::<List>()?;
#     l.as_mut().reverse();
#     i.stack_push(l);
#     Ok(())
# }

# /// list list `list-append` -> list : append two lists.
# pub fn list_append(i: &mut Interpreter) -> BuiltinRet {
#     let mut b = i.stack_pop::<List>()?;
#     let a = i.stack_pop::<List>()?;
#     b.as_mut().prepend(a.into_inner());
#     i.stack_push(b);
#     Ok(())
# }

# /// list n `list-split-at` -> list-tail list-head : split a list into two at index n.
# /// 0-indexed, negative numbers are from the other end of the list,
# /// and out of range indexes are saturated so that one of the lists is empty.
# pub fn list_split_at(i: &mut Interpreter) -> BuiltinRet {
#     let n = i.stack_pop::<i64>()?.into_inner();
#     let mut l = i.stack_pop::<List>()?;
#     let len = l.as_ref().len() as i64;
#     let n = if n < 0 { len + n } else { n };
#     let n = if n < 0 { 0 } else if n > len { len } else { n };
#     let head = l.as_mut().pop_n(n as usize);
#     i.stack_push(l);
#     i.stack_push(head);
#     Ok(())
# }


