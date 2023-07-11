
(import ./data)

(defn- wrap-value [sp sl sc v ep el ec]
  (data/meta-set (data/val v)
                 {:span {:from {:pos sp :l sl :c sc}
                         :to {:pos ep :l el :c ec}}}))

(defn- wrap-list [s l e]
  (data/meta-set (data/val l)
                 {:delimited [s e]}))

(defn- atomic [s]
  (let [[ok i] (protect (int/s64 s))]
    (if ok i
      (if-let [n (scan-number s)]
        n (symbol s)))))

(def worst-grammar
  (peg/compile
    ~{:comment (sequence (choice ";" "#!") (thru "\n"))

      :string (sequence
                "\""
                (accumulate (any (choice (if-not (set `\"`) '1)
                                          :string-escape)))
                "\"")

      :string-escape (sequence `\` (choice (replace "e" "\e")
                                           (replace "n" "\n")
                                           (replace "r" "\r")
                                           (replace "t" "\t")
                                           (sequence (constant `\`) 1)))

      :list (choice (replace (sequence '"(" (group :value*) '")") ,wrap-list)
                    (replace (sequence '"[" (group :value*) '"]") ,wrap-list)
                    (replace (sequence '"(" (group :value*) '")") ,wrap-list))

      :bool (choice (replace (capture "#t") true)
                    (replace (capture "#f") false))
      # :number (number (sequence (opt "-") :d* (opt ".") :d+))

      :atom (replace '(to (choice (set "\";()[]{}") :s)) ,atomic)

      :value (replace (sequence
                        (position) (line) (column)
                        (choice :string :list :bool :atom)
                        (position) (line) (column))
                      ,wrap-value)
      :value* (any (choice :s+ :comment :value))

      :main (sequence :value* (position) (line) (column))}))

(def Reader (data/new-type @{:name :reader}))

(defn reader [&named source]
  (data/construct Reader
                  @{:in ""
                    :pos {:pos 0 :c 1 :l 1}
                    :source source}))

(defn read [r input]
  (let [instr (string (r :in) input)
        res (peg/match worst-grammar instr)
        c (array/pop res)
        l (array/pop res)
        pos (array/pop res)
        newinstr (string/slice instr pos)
        # keep track of pos/line/col
        {:pos rpos :l rl :c rc} (r :pos)
        pos (+ rpos pos)
        l (+ rl l -1)
        c (if (= l rl) (+ rc c -1) c)]
    (put r :in newinstr)
    (put r :pos {:pos pos :l l :c c})
    (map (fn [x] (data/meta-set x {:source (r :source)})) res)))

(defn check [r &named raise]
  (default raise true)
  (let [s (r :in)
        {:l rl :c rc} (r :pos)]
    (unless (= s "")
      ((if raise errorf string/format)
         "Parse error at %s@%q:%q: %s"
              (or (r :source) "<???>")
              rl rc
              (if (> (length s) 40)
                (string (string/slice s 0 35) "...")
                s)))))

(defn read-file [filename]
  (let [f (file/open filename)
        buf (defer (file/close f) (file/read f :all))
        r (reader :source filename)
        res (read r buf)]
    (check r)
    res))

