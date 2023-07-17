
(import ../interpreter :as I)
(import ../data)

(defn file-open-options :builtin {:o 1} [i] [""])

(defn file-open-options-set-read :builtin {:i [:string] :o [:string]} [i o]
  [(string/join [o "r"])])

(defn file-open :builtin {:i [:string :string] :o [:val]} [i path opts]
  (let [[ok ret] (protect (file/open path (keyword (string/join [opts "n"]))))]
    [(if ok
       (data/new-port ret)
       (data/set-error ret))]))

    # util::add_const_type_builtin::<fs::OpenOptions>(i, "<file-open-options>");
    # i.add_builtin("file-open-options-set-append", |i: &mut Interpreter| {
    #     with_open_options(i, fs::OpenOptions::append)
    # });
    # i.add_builtin("file-open-options-set-create", |i: &mut Interpreter| {
    #     with_open_options(i, fs::OpenOptions::create)
    # });
    # i.add_builtin("file-open-options-set-create-new", |i: &mut Interpreter| {
    #     with_open_options(i, fs::OpenOptions::create_new)
    # });
    # i.add_builtin("file-open-options-set-truncate", |i: &mut Interpreter| {
    #     with_open_options(i, fs::OpenOptions::truncate)
    # });
    # i.add_builtin("file-open-options-set-write", |i: &mut Interpreter| {
    #     with_open_options(i, fs::OpenOptions::write)
    # });

    # i.add_builtin("file-open", |i: &mut Interpreter| {
    #     let opts = i.stack_pop::<fs::OpenOptions>()?;
    #     let path = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(opts.as_ref().open(path.as_ref())
    #                         .map(File::new).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });

    # util::add_const_type_builtin::<File>(i, "<file-port>");
    # i.add_builtin("file-port->string", util::port_to_string::<File>);
    # i.add_builtin("file-port-read-range", util::port_read_range::<File>);
    # i.add_builtin("file-port-write-range", util::port_write_range::<File>);
    # i.add_builtin("file-port-flush", util::port_flush::<File>);
    # i.add_builtin("fs-path-canonical", |i: &mut Interpreter| {
    #     let p = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::canonicalize(p.as_ref()).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });

    # i.add_builtin("fs-copy", |i: &mut Interpreter| {
    #     let dest = i.stack_pop::<PathBuf>()?;
    #     let src = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::copy(src.as_ref(), dest.as_ref())
    #                         .map(|_len| dest).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });
    # i.add_builtin("fs-move", |i: &mut Interpreter| {
    #     let dest = i.stack_pop::<PathBuf>()?;
    #     let src = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::rename(src.as_ref(), dest.as_ref())
    #                         .map(|_len| dest).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });

    # i.add_builtin("fs-file-delete", |i: &mut Interpreter| {
    #     let path = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::remove_file(path.as_ref()).map(|()| true).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });
    # i.add_builtin("fs-dir-delete-empty", |i: &mut Interpreter| {
    #     let path = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::remove_dir(path.as_ref()).map(|()| true).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });
    # i.add_builtin("fs-dir-delete", |i: &mut Interpreter| {
    #     let path = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::remove_dir_all(path.as_ref()).map(|()| true).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });

    # i.add_builtin("fs-dir-create", |i: &mut Interpreter| {
    #     let name = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::create_dir(name.as_ref()).map(|()| true).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });
    # i.add_builtin("fs-dir-create-path", |i: &mut Interpreter| {
    #     let name = i.stack_pop::<PathBuf>()?;
    #     i.stack_push_result(fs::create_dir_all(name.as_ref()).map(|()| true).map_err(|e| format!("{}", e)));
    #     Ok(())
    # });

    # i.add_builtin("fs-dir-entries", |i: &mut Interpreter| {
    #     let name = i.stack_pop::<PathBuf>()?;
    #     match fs::read_dir(name.as_ref()) {
    #         Ok(rd) => {
    #             let mut l = vec![];
    #             for f in rd {
    #                 if let Some(f) = util::or_io_error(i, f) {
    #                     l.push(Val::from(f.path()));
    #                 } else {
    #                     todo!("error");
    #                     // return;
    #                 }
    #             }
    #             i.stack_push(List::from(l));
    #         },
    #         Err(e) => i.stack_push_error(format!("{}", e)),
    #     }
    #     Ok(())
    # });

    # // i.add_builtin("fs-metadata");
    # // i.add_builtin("fs-link-target");
    # // i.add_builtin("fs-link-metadata");




