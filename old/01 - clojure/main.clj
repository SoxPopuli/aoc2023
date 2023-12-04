; --- Part 1 ---

(defn get-first-and-last [lst]
  (str (first lst) (last lst)))

(defn get-numbers [^String s]
  (->> s
       (filter #(Character/isDigit %))
       get-first-and-last
       Integer/parseInt))

(def input (line-seq (java.io.BufferedReader. *in*)))

(defn part1 []
  (->> input
       (map get-numbers)
       (reduce +)))

;(println (part1))
; --- Part 2 ---

(defn get-first-and-last-2 [line]
  (let [lst
        (re-seq #"(?=(zero|one|two|three|four|five|six|seven|eight|nine|\d))" line)]

    (->> group
         last
         (list (first group))
         (let [group (vec (map second lst))]
           (println (str line ": " group))))))

(defn words-to-digits [lst]
  (let
   [word-map {"zero" 0 "one" 1 "two" 2 "three" 3 "four" 4 "five" 5 "six" 6 "seven" 7 "eight" 8 "nine" 9}]

    #_{:clj-kondo/ignore [:inline-def]}
    (defn word-to-digit [word?]
      (let [digit? (get word-map word?)]
        (if (not= digit? nil)
          digit?
          word?)))

    (map word-to-digit lst)))

(defn part2 []
  (->> input
       (map get-first-and-last-2)
       (map words-to-digits)
       (map #(str (first %) (last %)))
       (map #(Integer/parseInt %))
       (reduce +)))

(println (part2))
