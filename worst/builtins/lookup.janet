
(import ../interpreter :as I)
(import ../data)

(defn make-lookup :builtin [i]
  (I/stack-push i (data/val @{})))

(defn lookup-insert :builtin {:i [data/Lookup :any :val] :o [data/Lookup]} [i m k v]
  (data/lookup-insert m k v)
  [m])

    # i.add_builtin("i64map-contains", |i: &mut Interpreter| {
    #     let k = i.stack_pop::<i64>()?.into_inner();
    #     let map = i.stack_pop::<I64Map>()?;
    #     i.stack_push(map.as_ref().data.contains_key(&k));
    #     Ok(())
    # });
    # i.add_builtin("i64map-get", |i: &mut Interpreter| {
    #     let k = i.stack_pop::<i64>()?.into_inner();
    #     let map = i.stack_pop::<I64Map>()?;
    #     i.stack_push_option(map.as_ref().data.get(&k).cloned());
    #     Ok(())
    # });
    # i.add_builtin("i64map-empty", |i: &mut Interpreter| {
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push(ht.as_ref().data.is_empty());
    #     Ok(())
    # });
    # i.add_builtin("i64map-length", |i: &mut Interpreter| {
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push(ht.as_ref().data.len() as i64);
    #     Ok(())
    # });
    # i.add_builtin("i64map-keys", |i: &mut Interpreter| {
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push(List::from(ht.as_ref().data.keys().cloned().map(Val::from).collect::<Vec<_>>()));
    #     Ok(())
    # });
    # i.add_builtin("i64map-min-key", |i: &mut Interpreter| {
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push_option(ht.as_ref().data.get_min().map(|(k, _)| *k));
    #     Ok(())
    # });
    # i.add_builtin("i64map-max-key", |i: &mut Interpreter| {
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push_option(ht.as_ref().data.get_max().map(|(k, _)| *k));
    #     Ok(())
    # });
    # i.add_builtin("i64map-next-key", |i: &mut Interpreter| {
    #     let k = i.stack_pop::<i64>()?.into_inner();
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push_option(ht.as_ref().data.get_next(&k).map(|(k, _)| *k));
    #     Ok(())
    # });
    # i.add_builtin("i64map-prev-key", |i: &mut Interpreter| {
    #     let k = i.stack_pop::<i64>()?.into_inner();
    #     let ht = i.stack_pop::<I64Map>()?;
    #     i.stack_push_option(ht.as_ref().data.get_prev(&k).map(|(k, _)| *k));
    #     Ok(())
    # });

