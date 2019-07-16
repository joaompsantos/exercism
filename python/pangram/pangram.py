def is_pangram(sentence):
    if not sentence.isascii() :
        raise Exception('Not valid characthers')

    pangram = True

    for letter in range(97,123):
        if sentence.lower().find(chr(letter)) == -1 :
            pangram = False

    return pangram
