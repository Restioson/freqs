# freqs

Find frequency of a set list of words in a directory of plaintext files. Splits by unicode whitespace 
and then removes ascii punctuation. Invoke with `freqs path_to_txts/ word1 word2 word3` and so on. The output is CSV,
separated by commas, and sorted first by the alphabetical order of the filenames, and then by the frequency of the
words. Words that don't exist in a certain file are still added to the output, with a count of 0. There is a header row
in the output CSV.
