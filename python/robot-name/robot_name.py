import random, string, numbers

class Robot(object):
    """Robot Class"""

    NNUM = 3
    NLET = 2

    # Save used names
    names_used = set()

    def __init__(self):
        name = self.randname()
            
        while name in Robot.names_used :
            name = self.randname()    

        self.name = name


    def reset(self):
        self.__init__()


    def randname(self):
        return ''.join([random.SystemRandom().choice(string.ascii_uppercase)for _ in range(Robot.NLET)]) + ''.join([ str(random.randint(0,9)) for _ in range(Robot.NNUM) ])
