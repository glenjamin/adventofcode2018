(ns day3.core
  (:gen-class))

(def LINE_REGEX #"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")

(defn parse-claim-line [line]
  (if-let [[_ id x y w h] (re-matches LINE_REGEX line)]
    {:id (Integer/parseInt id)
     :x (Integer/parseInt x)
     :y (Integer/parseInt y)
     :w (Integer/parseInt w)
     :h (Integer/parseInt h)}))

(defn read-claims [stream]
  (->>
    stream
    (java.io.BufferedReader.)
    (line-seq)
    (map parse-claim-line)
    (remove nil?)))

(defn expand-claim [{:keys [id x y w h]}]
  (into {} (for [x (range x (+ x w))
                 y (range y (+ y h))]
             [[x y] [id]])))

(defn apply-claims [fabric claims]
  (let [[claim & claims] claims
        fabric (merge-with concat fabric (expand-claim claim))]
    (if (seq claims)
      (recur fabric claims)
      fabric)))

(defn count-overlaps [fabric]
  (->> fabric
       (filter #(>= (count (val %)) 2))
       (count)))

(defn find-clear [fabric]
  (reduce
   (fn [acc ids]
     (if (= 1 (count ids))
       (conj acc (first ids))
       (apply disj acc ids)))
   #{}
   (sort-by count (vals fabric))))

(defn main [stream]
  (let [claims (read-claims stream)
        fabric (apply-claims {} claims)]
    (println "Overlaps: " (count-overlaps fabric))
    (println "Clear: " (find-clear fabric))))

(def small-claims
  [(parse-claim-line "#1 @ 1,3: 4x4")
   (parse-claim-line "#2 @ 3,1: 4x4")
   (parse-claim-line "#3 @ 5,5: 2x2")])

(defn -main
  [& args]
  (main *in*))
