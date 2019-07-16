def is_armstrong_number(number):
    numsum = 0
    power = len(str(number))
    aux = number

    while( aux != 0 ) :
        numsum = numsum + (( aux % 10 ) ** power)
        aux //= 10

    return numsum == number
